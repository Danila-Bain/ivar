/// Implementation of arithmetical operations for `f64` with different rounding modes.
/// 
/// This module implements arithmetical operations for `f64` with set rounding towards $+\infty$ (round up: "ru") or towards $-\infty$ (round down: "rd"). This is achieved in a slow but safe way by setting rounding mode flags in `MXCSR` register. The slowness comes from the fact that this register can only be set from memory, so each call to this function requires several memory operations, making it much slower than regular operations that have rounding to the nearest representable numbers by default.
///
/// The main purpose of this module is to provide rounded arithmetical operations as reference for testing their faster implementations.
/// ```
/// use ivar::rounded_arithmetic::mxcsr::add_ru_mxcsr;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one + pos_eps, one);
/// assert_eq!(add_ru_mxcsr(one, pos_eps), one.next_up());
///
/// assert_eq!(one + neg_eps, one);
/// assert_eq!(add_ru_mxcsr(one, neg_eps), one);
/// ```
#[rustfmt::skip]
pub mod mxcsr {
    use core::arch::asm;

    /// Implement arithmetical operations with different rounding modes set in `mxcsr` register.
    macro_rules! impl_op_mxcsr_round {
        ($type:ty, $op:literal, $f:ident ($x:ident $(,$y:ident)*), $instruction:literal, ru) => {
            impl_op_mxcsr_round!($type, $op, $f($x $(,$y)*), $instruction, "up", "24448");
        };
        ($type:ty, $op:literal, $f:ident ($x:ident $(,$y:ident)*), $instruction:literal, rd) => {
            impl_op_mxcsr_round!($type, $op, $f($x $(,$y)*), $instruction, "down", "16256");
        };
        ($type:ty, $op:literal, $f:ident ($x:ident $(,$y:ident)*),
            $instruction:literal, $rounding:literal, $mxcsr_value:literal) => {
            #[doc = concat!(
                "Perform the ", $op, " rounding ", $rounding, ".\n",
            )]
            #[must_use]
            pub fn $f(mut $x: $type, $($y: $type,)*) -> f64 {
                // SAFETY: We restore the value of `mxcsr` register after performing the arithmetic operation
                unsafe {
                    asm!(
                        "push {rax}",
                        "vstmxcsr [rsp]",
                        concat!("mov dword ptr [rsp + 4], ", $mxcsr_value),
                        "vldmxcsr [rsp + 4]",
                        $instruction,
                        "vldmxcsr [rsp]",
                        "pop {rax}",
                        $x = inout(xmm_reg) $x,
                        $($y = in(xmm_reg) $y,)*
                        rax = out(reg)_,
                        options(pure,nomem,preserves_flags)
                    );
                }
                $x
            }
        };
    }

    impl_op_mxcsr_round!(f64, "addition",      add_ru_mxcsr(x, y), "addsd {x}, {y}", ru);
    impl_op_mxcsr_round!(f64, "addition",      add_rd_mxcsr(x, y), "addsd {x}, {y}", rd);
    impl_op_mxcsr_round!(f64, "subtraction",   sub_ru_mxcsr(x, y), "subsd {x}, {y}", ru);
    impl_op_mxcsr_round!(f64, "subtraction",   sub_rd_mxcsr(x, y), "subsd {x}, {y}", rd);
    impl_op_mxcsr_round!(f64, "muliplication", mul_ru_mxcsr(x, y), "mulsd {x}, {y}", ru);
    impl_op_mxcsr_round!(f64, "muliplication", mul_rd_mxcsr(x, y), "mulsd {x}, {y}", rd);
    impl_op_mxcsr_round!(f64, "division",      div_ru_mxcsr(x, y), "divsd {x}, {y}", ru);
    impl_op_mxcsr_round!(f64, "division",      div_rd_mxcsr(x, y), "divsd {x}, {y}", rd);
}

/// Returns `x + y` and its exact rounding error.
///
/// The return value is pair `(RN(x+y), (x+y) - RN(x+y))`, where `+` denotes exact multiplication of reals and `RN` denotes the rounding of a real number to the nearest floating-point number.
///
/// It is proven that the rounding error `(x+y) - RN(x+y)` is representable as a floating-point number if `x` and `y` are.
///
/// In literature this is referred to as the 2Sum algorithm (see [Handbook of Floating-Point Arithmetic (2018), Algorithm 4.4](https://doi.org/10.1007/978-3-319-76526-6)).
/// ```
/// use ivar::rounded_arithmetic::add_lossless;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one, one + pos_eps);
/// assert_eq!(add_lossless(one, pos_eps), (one, pos_eps));
///
/// assert_eq!(one, one + neg_eps);
/// assert_eq!(add_lossless(one, neg_eps), (one, neg_eps));
/// ```
#[must_use]
pub const fn add_lossless(x: f64, y: f64) -> (f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    // The algorithm is optimal both in terms of number of floating point operations
    // and depth of the value dependency tree, unless there is a primary knowledge
    // about ordering of the inputs or branching is allowed.
    // (see [Handbook of Floating-Point Arithmetic (2018), Theorems 4.4 and 4.5](https://doi.org/10.1007/978-3-319-76526-6))
    let sum = x + y;
    let yy = sum - x;
    let xx = sum - yy;
    let err = (x - xx) + (y - yy);
    (sum, err)
}

/// Returns `x + y` and its exact rounding error, for `abs(x) >= abs(y)`.
///
/// The case `abs(x) < abs(y)` has undefined behaviour.
///
/// The return value is pair `(RN(x+y), (x+y) - RN(x+y))`, where `+` denotes exact multiplication of reals and `RN` denotes the rounding of a real number to the nearest floating-point number.
///
/// It is proven that the rounding error `(x+y) - RN(x+y)` is representable as a floating-point number if `x` and `y` are.
///
/// In literature this is referred to as the `Fast2Sum` algorithm (see [Handbook of Floating-Point Arithmetic (2018), Algorithm 4.3](https://doi.org/10.1007/978-3-319-76526-6)).
///
/// # Safety
/// This is a faster counterpart to [`add_lossless`] that uses just 3 additions/subtractons instead of 6, but requires its arguments to be ordered from largest to smallest in absolute value. Using wrong ordering leads to unaccounted rounding errors and the error term is not guaranteed to be equal (x+y) - RN(x + y).
#[must_use]
pub const unsafe fn add_lossless_fast(x: f64, y: f64) -> (f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    let sum = x + y;
    let err = y - (sum - x);
    (sum, err)
}

/// The `sub` version of [`add_lossless`], which acts the same as `add_lossless(x, -y)`.
///
/// ```
/// use ivar::rounded_arithmetic::sub_lossless;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one, one - pos_eps);
/// assert_eq!(sub_lossless(one, pos_eps), (one, -pos_eps));
///
/// assert_eq!(one, one - neg_eps);
/// assert_eq!(sub_lossless(one, neg_eps), (one, -neg_eps));
/// ```
#[must_use]
pub const fn sub_lossless(x: f64, y: f64) -> (f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    let dif = x - y;
    let yy = x - dif;
    let xx = dif + yy;
    let err = (x - xx) - (y - yy);
    (dif, err)
}

/// Returns `x * y` and its exact rounding error.
///
/// The return value is pair `(RN(x·y), (x·y) - RN(x·y))`, where `·` denotes exact multiplication of reals and `RN` denotes the rounding of a real number to the nearest floating-point number.
///
/// It is proven that the rounding error `(x·y) - RN(x·y)` is representable as a floating-point number if `x` and `y` are.
///
/// In literature this is referred to as the 2MultFMA algorithm (see [Handbook of Floating-Point Arithmetic (2018), Algorithm 4.8](https://doi.org/10.1007/978-3-319-76526-6)).
/// ```
/// use ivar::rounded_arithmetic::mul_lossless;
///
/// // difference of squares formula: (2^32 - 1)*(2^32 + 1) = 2^64 - 1
/// assert_eq!(mul_lossless(2f64.powi(32) - 1., 2f64.powi(32) + 1.), (2f64.powi(64), -1.));
/// ```
#[must_use]
pub const fn mul_lossless(x: f64, y: f64) -> (f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    let mul = x * y;
    let err = x.mul_add(y, -mul); // x * y - mul
    (mul, err)
}

/// For input values `x` and `y` computes the pair `(div, err)`, where `div = x / y` and `err` represents rounding error, such that in a sense of real numbers represented by corresponding f64-values, the equality `div + err/abs(y) == x/y` holds exactly
#[must_use]
pub const fn div_lossless(x: f64, y: f64) -> (f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    let div = x / y;
    let err = (-y).mul_add(div, x);
    // err * sign(y)
    let err_with_corrected_sign = f64::from_bits(err.to_bits() ^ (y.to_bits() & f64::SIGN_MASK));
    (div, err_with_corrected_sign)
}

/// Returns `a*x + y` with exact rounding errors.
///
/// If `(fma, err_1, err_2) = mul_add_lossless(a, x, y)`, then the equality `fma + err_1 + err_2 = a*x + y` holds exactly, where `+` and `*` denote addition and multiplication of the corresponding real numbers, not floating-point operations.
#[must_use]
pub const fn mul_add_lossless(a: f64, x: f64, y: f64) -> (f64, f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    let ax_plus_y = f64::mul_add(a, x, y);
    let (ax, ax_err) = mul_lossless(a, x);
    let (ax_err_plus_y, ax_err_plus_y_err) = add_lossless(y, ax_err);
    let (ax_plus_y_2, ax_plus_y_2_err) = add_lossless(ax, ax_err_plus_y);
    let ax_plus_y_err_0 = (ax_plus_y_2 - ax_plus_y) + ax_plus_y_2_err;
    // SAFETY: it is proven that the first term has larger absolute value
    let (ax_plus_y_err_1, ax_plus_y_err_2) =
        unsafe { add_lossless_fast(ax_plus_y_err_0, ax_err_plus_y_err) };
    (ax_plus_y, ax_plus_y_err_1, ax_plus_y_err_2)
}

/// For input value `x` computes the pair `(sqrt, err)`, where `sqrt` is rounded-to-nearest square root of `x` and `err` represents rounding error, such that in a sense of real numbers represented by corresponding f64-values, the equality `sqrt^2 + err == x` holds exactly.
#[must_use]
pub fn sqrt_lossless(x: f64) -> (f64, f64) {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest is assumed by algorithm"
    )]
    let sqrt = x.sqrt();
    let err = sqrt.mul_add(-sqrt, x); // x - sqrt^2
    (sqrt, err)
}

/// For input value `x` compute the pair `(cbrt, err)`, where `cbrt` is rounded-to-nearest cube root of `x` and `err` are rounding errors such that `cbrt^3 + err[0] + ... + err[4] == x` holds exactly.
#[must_use]
pub fn cbrt_lossless(x: f64) -> (f64, [f64; 5]) {
    #![allow(
        clippy::similar_names,
        reason = "names reflect structure, the alternative is to use separate leter for each variable, which is worse"
    )]
    // Note: this is naive implementation and I believe it could be significantly improved by analyzing the sizes of the error terms, such that they can be combined without rounding. I believe just two error terms are enough, as in `mul_add`.
    let y = x.cbrt();
    let (yy, yye) = mul_lossless(y, y);
    let (yyy, yyye) = mul_lossless(yy, y);
    let (yye_y, yye_y_e) = mul_lossless(yye, y);
    // so y^3 == yyy + yyye + yye_y + yye_y_e
    let (e, e_e) = sub_lossless(yyy, x);
    (y, [e, e_e, yyye, yye_y, yye_y_e])
}

#[must_use]
pub const fn from_2f64_ru((value, err): (f64, f64)) -> f64 {
    if err > 0. { value.next_up() } else { value }
}
#[must_use]
pub const fn from_2f64_rd((value, err): (f64, f64)) -> f64 {
    if err < 0. { value.next_down() } else { value }
}

/// Returns `x + y` rounded towards `+∞`.
///
/// The result is the least floating-point number that is guaranteed to be larger or equal to the exact sum `x + y`.
///
/// # Examples
/// ```
/// use ivar::rounded_arithmetic::add_ru;
/// use ivar::rounded_arithmetic::mxcsr::add_ru_mxcsr;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one + pos_eps, one);
/// assert_eq!(add_ru(one, pos_eps), one.next_up());
///
/// assert_eq!(one + neg_eps, one);
/// assert_eq!(add_ru(one, neg_eps), one);
///
/// fn check(x: f64, y: f64) {
///     let sum = add_ru(x, y);
///     let sum_mxcsr = add_ru_mxcsr(x, y);
///     assert_eq!(sum.is_nan(), sum_mxcsr.is_nan());
///     if !sum.is_nan() {
///          assert_eq!(add_ru(x, y), add_ru_mxcsr(x, y));
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn add_ru(x: f64, y: f64) -> f64 {
    from_2f64_ru(add_lossless(x, y))
}

/// Returns `x + y` rounded towards `-∞`.
///
/// The result is the largest floating-point number that is guaranteed to be less or equal to the exact sum `x + y`.
///
/// # Examples
/// ```
/// use ivar::rounded_arithmetic::add_rd;
/// use ivar::rounded_arithmetic::mxcsr::add_rd_mxcsr;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one + pos_eps, one);
/// assert_eq!(add_rd(one, pos_eps), one);
///
/// assert_eq!(one + neg_eps, one);
/// assert_eq!(add_rd(one, neg_eps), one.next_down());
///
/// fn check(x: f64, y: f64) {
///     let sum = add_rd(x, y);
///     let sum_mxcsr = add_rd_mxcsr(x, y);
///     assert_eq!(sum.is_nan(), sum_mxcsr.is_nan());
///     if !sum.is_nan() {
///          assert_eq!(add_rd(x, y), add_rd_mxcsr(x, y), "arguments were {}, {}", x, y);
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn add_rd(x: f64, y: f64) -> f64 {
    from_2f64_rd(add_lossless(x, y))
}

/// Returns `x - y` rounded towards `+∞`.
///
/// The result is the least floating-point number that is guaranteed to be larger or equal to the exact difference `x - y`.
/// ```
/// use ivar::rounded_arithmetic::sub_ru;
/// use ivar::rounded_arithmetic::mxcsr::sub_ru_mxcsr;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one + pos_eps, one);
/// assert_eq!(sub_ru(one, pos_eps), one);
///
/// assert_eq!(one + neg_eps, one);
/// assert_eq!(sub_ru(one, neg_eps), one.next_up());
///
/// fn check(x: f64, y: f64) {
///     let dif = sub_ru(x, y);
///     let dif_mxcsr = sub_ru_mxcsr(x, y);
///     assert_eq!(dif.is_nan(), dif_mxcsr.is_nan());
///     if !dif.is_nan() {
///          assert_eq!(sub_ru(x, y), sub_ru_mxcsr(x, y));
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn sub_ru(x: f64, y: f64) -> f64 {
    from_2f64_ru(sub_lossless(x, y))
}

/// ```
/// use ivar::rounded_arithmetic::sub_rd;
/// use ivar::rounded_arithmetic::mxcsr::sub_rd_mxcsr;
///
/// let one = 1_f64;
/// let pos_eps = 1e-17_f64;
/// let neg_eps = -pos_eps;
///
/// assert_eq!(one - pos_eps, one);
/// assert_eq!(sub_rd(one, pos_eps), one.next_down());
///
/// assert_eq!(one - neg_eps, one);
/// assert_eq!(sub_rd(one, neg_eps), one);
///
/// fn check(x: f64, y: f64) {
///     let dif = sub_rd(x, y);
///     let dif_mxcsr = sub_rd_mxcsr(x, y);
///     assert_eq!(dif.is_nan(), dif_mxcsr.is_nan());
///     if !dif.is_nan() {
///          assert_eq!(sub_rd(x, y), sub_rd_mxcsr(x, y), "arguments were {}, {}", x, y);
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn sub_rd(x: f64, y: f64) -> f64 {
    from_2f64_rd(sub_lossless(x, y))
}

/// ```
/// use ivar::rounded_arithmetic::mul_ru;
/// use ivar::rounded_arithmetic::mxcsr::mul_ru_mxcsr;
///
/// fn check(x: f64, y: f64) {
///     let mul = mul_ru(x, y);
///     let mul_mxcsr = mul_ru_mxcsr(x, y);
///     assert_eq!(mul.is_nan(), mul_mxcsr.is_nan());
///     if !mul.is_nan() {
///          assert_eq!(mul_ru(x, y), mul_ru_mxcsr(x, y));
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn mul_ru(x: f64, y: f64) -> f64 {
    from_2f64_ru(mul_lossless(x, y))
}

/// ```
/// use ivar::rounded_arithmetic::mul_rd;
/// use ivar::rounded_arithmetic::mxcsr::mul_rd_mxcsr;
///
/// fn check(x: f64, y: f64) {
///     let mul = mul_rd(x, y);
///     let mul_mxcsr = mul_rd_mxcsr(x, y);
///     assert_eq!(mul.is_nan(), mul_mxcsr.is_nan());
///     if !mul.is_nan() {
///          assert_eq!(mul_rd(x, y), mul_rd_mxcsr(x, y), "arguments were {}, {}", x, y);
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn mul_rd(x: f64, y: f64) -> f64 {
    from_2f64_rd(mul_lossless(x, y))
}
/// ```
/// use ivar::rounded_arithmetic::div_ru;
/// use ivar::rounded_arithmetic::mxcsr::div_ru_mxcsr;
///
/// fn check(x: f64, y: f64) {
///     let div = div_ru(x, y);
///     let div_mxcsr = div_ru_mxcsr(x, y);
///     assert_eq!(div.is_nan(), div_mxcsr.is_nan());
///     if !div.is_nan() {
///          assert_eq!(div_ru(x, y), div_ru_mxcsr(x, y));
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn div_ru(x: f64, y: f64) -> f64 {
    from_2f64_ru(div_lossless(x, y))
}

/// ```
/// use ivar::rounded_arithmetic::div_rd;
/// use ivar::rounded_arithmetic::div_lossless;
/// use ivar::rounded_arithmetic::mxcsr::div_rd_mxcsr;
///
/// fn check(x: f64, y: f64) {
///     let div = div_rd(x, y);
///     let div_mxcsr = div_rd_mxcsr(x, y);
///     assert_eq!(div.is_nan(), div_mxcsr.is_nan());
///     if !div.is_nan() {
///          assert_eq!(div_rd(x, y), div_rd_mxcsr(x, y), "arguments were {}, {}; div_lossless is {:?}", x, y, div_lossless(x, y));
///     }
/// }
///
/// let values = [
///     0.0, 1., 1. + f64::EPSILON, 2., 2. + f64::EPSILON, 10., 42.,
///     f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
///     1.23e-4, 5.67e-8, 9.1011e-12, 1.31415e-16, 1e-17, 2e-17, 3e-17,
///     1.2e-34, 5.6e-78, 9.10e-111, 2.13e-141,
///     1.23e4, 5.67e8, 9.1011e12, 1.31415e16, 1.71819e20,
/// ];
///
/// for x in values.clone().into_iter() {
///    for y in values.into_iter() {
///        check(x, y);
///        check(x, -y);
///        check(-x, y);
///        check(-x, -y);
///    }
/// }
///
/// ```
#[must_use]
pub const fn div_rd(x: f64, y: f64) -> f64 {
    from_2f64_rd(div_lossless(x, y))
}

#[must_use]
pub fn sqrt_ru(x: f64) -> f64 {
    from_2f64_ru(sqrt_lossless(x))
}

#[must_use]
pub fn sqrt_rd(x: f64) -> f64 {
    from_2f64_rd(sqrt_lossless(x))
}

#[must_use]
pub fn cbrt_ru(x: f64) -> f64 {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest gives best estimation for the error sign"
    )]
    let (cbrt, err) = cbrt_lossless(x);
    from_2f64_ru((cbrt, (err[0] + err[1]) + (err[2] + err[3]) + err[4]))
}

#[must_use]
pub fn cbrt_rd(x: f64) -> f64 {
    #![expect(
        clippy::float_arithmetic,
        reason = "rounding to nearest gives best estimation for the error sign"
    )]
    let (cbrt, err) = cbrt_lossless(x);
    from_2f64_rd((cbrt, (err[0] + err[1]) + (err[2] + err[3]) + err[4]))
}
