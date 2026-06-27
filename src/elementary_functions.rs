/// Implementation of basic interval operations
pub(crate) mod basic_operations {
    use crate::{
        Interval,
        rounded_arithmetic::{
            add_rd, add_ru, cbrt_rd, cbrt_ru, div_lossless, div_rd, div_ru, from_2f64_rd,
            from_2f64_ru, mul_add_lossless, mul_lossless, mul_rd, mul_ru, sqrt_rd, sqrt_ru, sub_rd,
            sub_ru,
        },
    };
    use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

    #[must_use]
    /// Returns minimum and maximum values in array of four pairs.
    ///
    /// Default comparison is used, so min and max are found using  lexicographical order.
    pub(crate) const fn min_max_4_pairs(mut arr: [(f64, f64); 4]) -> ((f64, f64), (f64, f64)) {
        if arr[0] > arr[1] {
            arr.swap(0, 1);
        }
        if arr[2] > arr[3] {
            arr.swap(2, 3);
        }
        if arr[0] > arr[2] {
            arr.swap(0, 2);
        }
        if arr[3] < arr[1] {
            arr.swap(3, 1);
        }

        (arr[0], arr[3])
    }

    impl Interval {
        /// Negate the interval.
        ///
        /// This is `const` version of [`Neg::neg`] implementation for `Interval`.
        ///
        /// [`Neg::neg`]: Interval#impl-Neg-for-Interval
        #[must_use]
        pub const fn neg(self) -> Self {
            #![expect(clippy::float_arithmetic, reason = "negation is exact")]
            Self(-self.1, -self.0)
        }
        #[must_use]
        pub const fn add(self, rhs: Self) -> Self {
            Self(add_rd(self.0, rhs.0), add_ru(self.1, rhs.1))
        }
        #[must_use]
        pub const fn sub(self, rhs: Self) -> Self {
            Self(sub_rd(self.0, rhs.1), sub_ru(self.1, rhs.0))
        }
        #[must_use]
        pub const fn mul(self, rhs: Self) -> Self {
            if self.is_nai() || rhs.is_nai() {
                Self::NAI
            } else if self.is_empty() || rhs.is_empty() {
                Self::EMPTY
            } else {
                let products = [
                    mul_lossless(self.0, rhs.0),
                    mul_lossless(self.1, rhs.0),
                    mul_lossless(self.0, rhs.1),
                    mul_lossless(self.1, rhs.1),
                ];
                let (inf_pair, sup_pair) = min_max_4_pairs(products);
                Self(from_2f64_rd(inf_pair), from_2f64_ru(sup_pair))
            }
        }
        #[must_use]
        pub const fn div(self, rhs: Self) -> Self {
            if self.is_nai() || rhs.is_nai() {
                Self::NAI
            } else if self.is_empty() || rhs.is_empty() {
                Self::EMPTY
            } else if rhs.contains(0.) {
                if self.contains(0.) {
                    Self::NAI
                } else {
                    Self::ENTIRE
                }
            } else {
                let divisions = [
                    div_lossless(self.0, rhs.0),
                    div_lossless(self.1, rhs.0),
                    div_lossless(self.0, rhs.1),
                    div_lossless(self.1, rhs.1),
                ];
                let (inf_pair, sup_pair) = min_max_4_pairs(divisions);
                Self(from_2f64_rd(inf_pair), from_2f64_ru(sup_pair))
            }
        }
        #[must_use]
        pub const fn rem(self, rhs: Self) -> Self {
            // excessive widening?
            self.div(rhs).fract().mul(rhs)
        }
    }

    impl Neg for Interval {
        type Output = Self;
        /// Negate the interval
        ///
        /// **Exact formula**: `-[a, b] = [-b, -a]`
        ///
        /// **Accuracy mode**: *tightest* (as IEEE 1788-2015 requires). This operation is **exact** and never rounds.
        ///
        /// For a `const` version of this operation, see [`Interval::neg`].
        ///
        /// # Examples
        /// ```
        /// use ivar::{Interval, iv};
        /// use std::f64::consts::PI;
        ///
        /// assert_eq!(-iv!(2.), iv!(-2.));
        /// assert_eq!(-(-iv!(0.1)), iv!(0.1));
        /// assert_eq!(-iv![2., 5.], iv![-5., -2.]);
        /// assert_eq!(-iv![-5., -2.], iv![2., 5.]);
        /// assert_eq!(-iv![-3., 2.], iv![-2., 3.]);
        ///
        /// // `neg` just flips the sign bit, so it is reflexive: no interval widening due to rounding occurs
        /// assert_eq!(-(-Interval::new(-3., PI)), Interval::new(-3., PI));
        ///
        /// assert_eq!(-Interval::new(0., f64::INFINITY), Interval::new(f64::NEG_INFINITY, 0.));
        /// assert_eq!(-Interval::EMPTY, Interval::EMPTY);
        /// assert_eq!(-Interval::ENTIRE, Interval::ENTIRE);
        /// assert!((-Interval::NAI).is_nai());
        /// ```
        fn neg(self) -> Self::Output {
            self.neg()
        }
    }
    impl Add<Self> for Interval {
        type Output = Self;

        /// Add two intervals
        ///
        /// *Exact formula*: `[a, b] + [c, d] = [a + c, b + d]`.
        ///
        /// *Accuracy mode*: *tightest* (as IEEE 1788-2015 requires). This operation rounds the exact value outwards.
        ///
        /// # Examples
        /// ```
        /// use ivar::iv;
        ///
        /// assert_eq!(iv![1., 2.] + iv![5., 6.], iv![6., 8.]);
        /// assert_eq!(iv![1., 2.] + iv![5., inf], iv![6., inf]);
        /// ```
        fn add(self, rhs: Self) -> Self::Output {
            self.add(rhs)
        }
    }
    impl Sub<Self> for Interval {
        type Output = Self;
        /// Subtract two intervals
        ///
        /// *Exact formula*: `[a, b] - [c, d] = [a - d, b - c]`.
        ///
        /// *Accuracy mode*: *tightest* (as IEEE 1788-2015 requires). This operation rounds the exact value outwards.
        ///
        /// # Examples
        /// ```
        /// use ivar::iv;
        ///
        /// assert_eq!(iv![1., 2.] - iv![5., 6.], iv![-5., -3.]);
        /// assert_eq!(iv![1., 2.] - iv![5., inf], iv![-inf, -3.]);
        /// ```
        fn sub(self, rhs: Self) -> Self::Output {
            self.sub(rhs)
        }
    }

    impl Mul<Self> for Interval {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            self.mul(rhs)
        }
    }
    impl Div<Self> for Interval {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self.div(rhs)
        }
    }

    impl Rem<Self> for Interval {
        type Output = Self;
        fn rem(self, rhs: Self) -> Self::Output {
            self.rem(rhs)
        }
    }

    /// Basic operations defined in Table 9.1. (Required forward elementary functions) of IEEE Std 1788-2015 beside [`Interval::neg`], [`Interval::add`], [`Interval::sub`], [`Interval::mul`], and [`Interval::div`].
    impl Interval {
        #[must_use]
        pub const fn recip(self) -> Self {
            if self.is_empty() {
                self
            } else if self.interior_contains(0.) {
                Self::ENTIRE
            } else if self.0 == 0. {
                if self.1 == 0. {
                    Self::NAI
                } else {
                    Self::POSITIVE
                }
            } else if self.1 == 0. {
                Self::NEGATIVE
            } else if self.0 > 0. {
                Self(div_rd(1., self.1), div_ru(1., self.0)) // positive endpoints: reverse order
            } else {
                Self(div_rd(1., self.0), div_ru(1., self.1)) // negative endpoints: preserve order
            }
        }

        #[must_use]
        pub const fn sqr(self) -> Self {
            if self.is_nai() {
                Self::NAI
            } else if self.is_empty() {
                Self::EMPTY
            } else if self.0 > 0. {
                Self(mul_rd(self.0, self.0), mul_ru(self.1, self.1))
            } else if self.1 < 0. {
                Self(mul_rd(self.1, self.1), mul_ru(self.0, self.0))
            } else {
                Self(0., f64::max(mul_ru(self.0, self.0), mul_ru(self.1, self.1)))
            }
        }

        #[must_use]
        pub fn sqrt(self) -> Self {
            if self.is_nai() {
                Self::NAI
            } else if self.is_empty() {
                Self::EMPTY
            } else if self.0 < 0. {
                Self::NAI
            } else {
                Self(sqrt_rd(self.0), sqrt_ru(self.1))
            }
        }
        #[must_use]
        pub fn cbrt(self) -> Self {
            if self.is_nai() {
                Self::NAI
            } else if self.is_empty() {
                Self::EMPTY
            } else {
                Self(cbrt_rd(self.0), cbrt_ru(self.1))
            }
        }

        #[must_use]
        pub fn mul_add(self, x: Self, y: Self) -> Self {
            if self.is_nai() || x.is_nai() || y.is_nai() {
                Self::NAI
            } else if self.is_empty() || x.is_empty() || y.is_empty() {
                Self::EMPTY
            } else {
                const fn min_triple(x: (f64, f64, f64), y: (f64, f64, f64)) -> (f64, f64, f64) {
                    if x < y { x } else { y }
                }
                const fn max_triple(x: (f64, f64, f64), y: (f64, f64, f64)) -> (f64, f64, f64) {
                    if x > y { x } else { y }
                }

                let lower = [
                    mul_add_lossless(self.0, x.0, y.0),
                    mul_add_lossless(self.1, x.0, y.0),
                    mul_add_lossless(self.0, x.1, y.0),
                    mul_add_lossless(self.1, x.1, y.0),
                ];
                let higher = [
                    mul_add_lossless(self.0, x.0, y.1),
                    mul_add_lossless(self.1, x.0, y.1),
                    mul_add_lossless(self.0, x.1, y.1),
                    mul_add_lossless(self.1, x.1, y.1),
                ];

                let inf = min_triple(
                    min_triple(lower[0], lower[1]),
                    min_triple(lower[2], lower[3]),
                );
                let sup = max_triple(
                    max_triple(higher[0], higher[1]),
                    max_triple(higher[2], higher[3]),
                );

                #[expect(
                    clippy::float_arithmetic,
                    reason = "rounding to nearest gives correct sign of error"
                )]
                Self(
                    from_2f64_rd((inf.0, inf.1 + inf.2)),
                    from_2f64_ru((sup.0, sup.1 + sup.2)),
                )
            }
        }

        #[must_use]
        pub fn hypot(self, other: Self) -> Self {
            (self.sqr() + other.sqr()).sqrt()
        }

        #[must_use]
        pub fn div_euclid(self, other: Self) -> Self {
            Self::hull(
                self.div(other.inter(Self::POSITIVE)).floor(),
                self.div(other.inter(Self::NEGATIVE)).ceil(),
            )
        }
    }
}

/// Implementation of power interval functions (`powi`, `pow`, exponents, logarithms)
mod power_functions {
    use crate::{
        Interval,
        rounded_arithmetic::{mul_rd, mul_ru},
    };

    /// Power functions
    impl Interval {
        #[must_use]
        pub fn powi(self, n: i32) -> Self {
            if self.is_empty() || n == 1 {
                self
            } else if n == 0 {
                Self(1., 1.)
            } else if n > 0 {
                if n % 2 == 0 {
                    if self.contains(0.) {
                        let mut sup = f64::max(self.0.abs(), self.1.abs());
                        for _ in 1..n {
                            sup = mul_ru(sup, self.1);
                        }
                        Self(0., sup)
                    } else if self.0 > 0. {
                        let mut inf = self.0;
                        let mut sup = self.1;
                        for _ in 1..n {
                            inf = mul_rd(inf, self.0);
                            sup = mul_ru(sup, self.1);
                        }
                        Self(inf, sup)
                    } else {
                        let mut inf = self.1;
                        let mut sup = self.0;
                        for _ in 1..n {
                            inf = mul_rd(inf, self.1);
                            sup = mul_ru(sup, self.0);
                        }
                        Self(inf, sup)
                    }
                } else {
                    let mut inf = self.0;
                    let mut sup = self.1;
                    for _ in 1..n {
                        inf = mul_rd(inf, self.0);
                        sup = mul_ru(sup, self.1);
                    }
                    Self(inf, sup)
                }
            } else {
                self.recip().powi(-n)
            }
        }
        #[must_use]
        pub fn pow(self, p: Self) -> Self {
            (self.ln() * p).exp()
        }
        #[must_use]
        pub fn exp(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::exp_rd(self.0), crlibm::exp_ru(self.1))
            }
        }
        #[must_use]
        pub fn exp2(self) -> Self {
            (self * Self::LN_2).exp()
        }
        #[must_use]
        pub fn exp10(self) -> Self {
            (self * Self::LN_10).exp()
        }
        #[must_use]
        pub fn exp_m1(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::exp_m1_rd(self.0), crlibm::exp_m1_ru(self.1))
            }
        }

        #[must_use]
        pub fn log(self, base: Self) -> Self {
            self.ln() / base.ln()
        }

        #[must_use]
        pub fn ln(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 < 0. {
                Self::NAI
            } else {
                Self(crlibm::ln_rd(self.0), crlibm::ln_ru(self.1))
            }
        }
        #[must_use]
        pub fn log2(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 < 0. {
                Self::NAI
            } else {
                Self(crlibm::log2_rd(self.0), crlibm::log2_ru(self.1))
            }
        }
        #[must_use]
        pub fn log10(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 < 0. {
                Self::NAI
            } else {
                Self(crlibm::log10_rd(self.0), crlibm::log10_ru(self.1))
            }
        }
        #[must_use]
        pub fn ln_1p(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 < 0. {
                Self::NAI
            } else {
                Self(crlibm::ln_1p_rd(self.0), crlibm::ln_1p_ru(self.1))
            }
        }
    }
}

/// Implementation of trigonometric and hyperbolic interval functions
pub(crate) mod trigonometric_and_hyperbolic_functions {
    use core::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        Interval,
        rounded_arithmetic::{
            add_rd, add_ru, div_rd, div_ru, mul_rd, mul_ru, sqrt_rd, sqrt_ru, sub_rd, sub_ru,
        },
    };

    impl Interval {
        //                                    //
        //                  •                 //
        //           •             •          //
        //        •         |         •       //
        //                  |                 //
        //                  |                 //
        //    •       1     |     0      •    //
        //                  |                 //
        //                  |                 //
        //   •  ------------|------------ •   //
        //                  |                 //
        //                  |                 //
        //    •       2     |     3      •    //
        //                  |                 //
        //                  |                 //
        //        •         |         •       //
        //           •             •          //
        //                  •                 //
        //                                    //

        /// Returns sine of `self`
        #[must_use]
        pub fn sin(self) -> Self {
            if self.is_empty() {
                return self;
            }
            //  .-----.
            // /   |   \
            // | 1 | 0 |
            // \   |  /
            //  ------
            let halves = (self / Self::PI + 0.5).floor();
            #[expect(
                clippy::float_arithmetic,
                clippy::modulo_arithmetic,
                reason = "halves.0.abs() is integer, so modulo 2 is exact and has value 0. or 1."
            )]
            let h_start = halves.0.abs() % 2.;
            #[expect(
                clippy::float_arithmetic,
                reason = "halves.1 and halves.0 are integers and in relevant cases `h_steps = 0` and `h_steps = 1` this subtraction is exact"
            )]
            let h_steps = halves.1 - halves.0;
            match (h_steps, h_start) {
                (0., 0.) => Self(crlibm::sin_rd(self.0), crlibm::sin_ru(self.1)),
                (0., 1.) => Self(crlibm::sin_rd(self.1), crlibm::sin_ru(self.0)),
                (1., 0.) => Self(crlibm::sin_rd(self.0).min(crlibm::sin_rd(self.1)), 1.),
                (1., 1.) => Self(-1., crlibm::sin_ru(self.0).max(crlibm::sin_ru(self.1))),
                (_, _) => Self(-1., 1.),
            }
        }
        /// Returns sine of `self × π`
        #[must_use]
        pub fn sin_pi(self) -> Self {
            if self.is_empty() {
                return self;
            }
            //  .-----.
            // /   |   \
            // | 1 | 0 |
            // \   |  /
            //  ------
            let halves = (self + 0.5).floor();
            #[expect(
                clippy::float_arithmetic,
                clippy::modulo_arithmetic,
                reason = "halves.0.abs() is integer, so modulo 2 is exact and has value 0. or 1."
            )]
            let h_start = halves.0.abs() % 2.;
            #[expect(
                clippy::float_arithmetic,
                reason = "halves.1 and halves.0 are integers and in relevant cases `h_steps = 0` and `h_steps = 1` this subtraction is exact"
            )]
            let h_steps = halves.1 - halves.0;
            match (h_steps, h_start) {
                (0., 0.) => Self(crlibm::sinpi_rd(self.0), crlibm::sinpi_ru(self.1)),
                (0., 1.) => Self(crlibm::sinpi_rd(self.1), crlibm::sinpi_ru(self.0)),
                (1., 0.) => Self(crlibm::sinpi_rd(self.0).min(crlibm::sinpi_rd(self.1)), 1.),
                (1., 1.) => Self(-1., crlibm::sinpi_ru(self.0).max(crlibm::sinpi_ru(self.1))),
                (_, _) => Self(-1., 1.),
            }
        }
        /// Returns cosine of `self`
        #[must_use]
        pub fn cos(self) -> Self {
            if self.is_empty() {
                return self;
            }
            //  .-----.
            // /   0   \
            // |-------|
            // \   1  /
            //  ------
            let halves = (self / Self::PI).floor();
            #[expect(
                clippy::float_arithmetic,
                clippy::modulo_arithmetic,
                reason = "halves.0.abs() is integer, so modulo 2 is exact and has value 0. or 1."
            )]
            let h_start = halves.0.abs() % 2.;
            #[expect(
                clippy::float_arithmetic,
                reason = "halves.1 and halves.0 are integers and in relevant cases `h_steps = 0` and `h_steps = 1` this subtraction is exact"
            )]
            let h_steps = halves.1 - halves.0;
            match (h_steps, h_start) {
                (0., 0.) => Self(crlibm::cos_rd(self.1), crlibm::cos_ru(self.0)),
                (0., 1.) => Self(crlibm::cos_rd(self.0), crlibm::cos_ru(self.1)),
                (1., 0.) => Self(-1., crlibm::cos_ru(self.0).max(crlibm::cos_ru(self.1))),
                (1., 1.) => Self(crlibm::cos_rd(self.0).min(crlibm::cos_rd(self.1)), 1.),
                (_, _) => Self(-1., 1.),
            }
        }
        /// Returns cosine of `self × π`
        #[must_use]
        pub fn cos_pi(self) -> Self {
            if self.is_empty() {
                return self;
            }
            //  .-----.
            // /   0   \
            // |-------|
            // \   1  /
            //  ------
            let halves = self.floor();
            #[expect(
                clippy::float_arithmetic,
                clippy::modulo_arithmetic,
                reason = "halves.0.abs() is non-negative integer, so modulo 2 is exact and has value 0. or 1."
            )]
            let h_start = halves.0.abs() % 2.;
            #[expect(
                clippy::float_arithmetic,
                reason = "halves.1 and halves.0 are integers and in relevant cases `h_steps = 0` and `h_steps = 1` this subtraction is exact"
            )]
            let h_steps = halves.1 - halves.0;
            match (h_steps, h_start) {
                (0., 0.) => Self(crlibm::cospi_rd(self.1), crlibm::cospi_ru(self.0)),
                (0., 1.) => Self(crlibm::cospi_rd(self.0), crlibm::cospi_ru(self.1)),
                (1., 0.) => Self(-1., crlibm::cospi_ru(self.0).max(crlibm::cospi_ru(self.1))),
                (1., 1.) => Self(crlibm::cospi_rd(self.0).min(crlibm::cospi_rd(self.1)), 1.),
                (_, _) => Self(-1., 1.),
            }
        }
        /// Returns pair `(self.sin(), self.cos())`
        #[must_use]
        pub fn sin_cos(self) -> (Self, Self) {
            // TODO: branch optimization
            (self.sin(), self.cos())
        }
        /// Returns tangent of `self`
        #[must_use]
        pub fn tan(self) -> Self {
            #[expect(clippy::float_cmp, reason = "compared values are integers")]
            if self.is_empty() {
                self
            } else if div_rd(sub_rd(self.0, FRAC_PI_2), PI).floor()
                != div_ru(sub_ru(self.1, FRAC_PI_2), PI).floor()
            {
                // self.0 and self.1 are in distinct intervals of continuity of tangent
                Self::ENTIRE
            } else {
                Self(crlibm::tan_rd(self.0), crlibm::tan_ru(self.1))
            }
        }
        /// Returns tangent of `self × π`
        #[must_use]
        pub fn tan_pi(self) -> Self {
            #[expect(clippy::float_cmp, reason = "compared values are integers")]
            if self.is_empty() {
                self
            } else if sub_rd(self.0, 0.5).floor() != sub_ru(self.1, 0.5).floor() {
                // self.0 and self.1 are in distinct intervals of continuity of tangent
                Self::ENTIRE
            } else {
                Self(crlibm::tanpi_rd(self.0), crlibm::tanpi_ru(self.1))
            }
        }
        /// Returns inverse sine of `self`.
        ///
        /// **Accuracy**: *tightest*.
        ///
        /// **IEEE 1788-2015 compliance**: _required_, required accuracy mode: *accurate*.
        ///
        /// # Examples
        /// ```
        /// use ivar::{Interval, iv};
        ///
        /// // asin([-1, 1]) = [-π/2, π/2]
        /// assert_eq!(iv!(-1., 1.).asin(), iv!(+- Interval::FRAC_PI_2.sup()));
        /// // asin([0, 1]) = [0, π/2]
        /// assert_eq!(iv!(0., 1.).asin(), iv!(0., Interval::FRAC_PI_2.sup()));
        /// // asin([-1, 0]) = [0π/2, 0]
        /// assert_eq!(iv!(-1., 0.).asin(), iv!(-Interval::FRAC_PI_2.sup(), 0.));
        ///
        ///
        /// // sin(asin([a, b])) = [a, b]
        /// assert_eq!(
        ///     iv!(0.3, 0.4).asin().sin(),
        ///     iv!(0.3, 0.4).next_n(-1, 1),
        /// );
        /// assert_eq!(
        ///     iv!(-0.6, -0.4).asin().sin(),
        ///     iv!(-0.6, -0.4).next_n(-1, 1),
        /// );
        /// assert_eq!(
        ///     iv!(0.5, 0.95).asin().sin(),
        ///     iv!(0.5, 0.95).next_n(-1, 1),
        /// );
        ///
        /// assert_eq!(
        ///     iv!(0.1, 1.).asin().sin(),
        ///     iv!(0.1, 1.).next_n(-1, 0),
        /// );
        /// assert_eq!(
        ///     iv!(0.01, 1.).asin().sin(),
        ///     iv!(0.01, 1.).next_n(-1, 0),
        /// );
        /// ```
        #[must_use]
        pub fn asin(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::asin_rd(self.0), crlibm::asin_ru(self.1))
            }
        }
        /// Returns inverse sine of `self`, which is then divided by `π`.
        ///
        /// **Accuracy**: *tightest*.
        ///
        /// **IEEE 1788-2015 compliance**: _recommended_, required accuracy mode: *accurate*.
        #[must_use]
        pub fn asin_pi(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::asinpi_rd(self.0), crlibm::asinpi_ru(self.1))
            }
        }
        /// Returns inverse cosine of `self`.
        ///
        /// **Accuracy**: *tightest*.
        ///
        /// **IEEE 1788-2015 compliance**: _required_, required accuracy mode: *accurate*.
        ///
        /// # Examples
        /// ```
        /// use ivar::{Interval, iv};
        ///
        /// // acos([-1, 1]) = [0, π]
        /// assert_eq!(iv!(-1., 1.).acos(), iv!(0., Interval::PI.sup()));
        /// // acos([0, 1]) = [0, π/2]
        /// assert_eq!(iv!(0., 1.).acos(), iv!(0., Interval::FRAC_PI_2.sup()));
        /// // acos([-1, 0]) = [π/2, π]
        /// assert_eq!(iv!(-1., 0.).acos(), iv!(Interval::FRAC_PI_2.inf(), Interval::PI.sup()));
        ///
        ///
        /// // cos(acos([a, b])) = [a, b]
        /// assert_eq!(
        ///     iv!(0.3, 0.4).acos().cos(),
        ///     iv!(0.3, 0.4).next_n(-2, 2),
        /// );
        /// assert_eq!(
        ///     iv!(-0.6, -0.4).acos().cos(),
        ///     iv!(-0.6, -0.4).next_n(-3, 1),
        /// );
        /// assert_eq!(
        ///     iv!(0.5, 0.95).acos().cos(),
        ///     iv!(0.5, 0.95).next_n(-2, 1),
        /// );
        ///
        /// assert_eq!(
        ///     iv!(0.1, 1.).acos().cos(),
        ///     iv!(0.1, 1.).next_n(-16, 0),
        /// );
        /// assert!(iv!(0.1, 1.).acos().cos().subset(iv!(0.1, 1.) + iv!(+- 1e-15)));
        ///
        /// // large number of representable numbers in between true and returned bound
        /// assert_eq!(
        ///     iv!(0.01, 1.).acos().cos(),
        ///     iv!(0.01, 1.).next_n(-76, 0),
        /// );
        /// // but absolute error is still small
        /// assert!(iv!(0.01, 1.).acos().cos().subset(iv!(0.01, 1.) + iv!(+- 1e-15)));
        /// ```
        #[must_use]
        pub fn acos(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::acos_rd(self.1), crlibm::acos_ru(self.0)) // note the reverse order
            }
        }
        /// Returns inverse cosine of `self`, which is then divided by `π`.
        ///
        /// **Accuracy**: *tightest*.
        ///
        /// **IEEE 1788-2015 compliance**: _recommended_, required accuracy mode: *accurate*.
        #[must_use]
        pub fn acos_pi(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::acospi_rd(self.1), crlibm::acospi_ru(self.0)) // note the reverse order
            }
        }
        /// Returns arctangent of `self`.
        ///
        /// **Accuracy**: *tightest*
        ///
        /// **IEEE 1788-2015 compliance**: _required_, required accuracy mode: *accurate*.
        #[must_use]
        pub fn atan(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::atan_rd(self.0), crlibm::atan_ru(self.1))
            }
        }
        /// Returns arctangent of `self`, which is then divided by `π`.
        ///
        /// **Accuracy**: *tightest*
        ///
        /// **IEEE 1788-2015 compliance**: _recommended_, required accuracy mode: *accurate*.
        #[must_use]
        pub fn atan_pi(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::atanpi_rd(self.0), crlibm::atanpi_ru(self.1))
            }
        }
        /// Returns argument range of vectors with coordinates `x ∈ other` and `y ∈ self`.
        ///
        /// **Accuracy**: *unknown*.
        ///
        /// **IEEE 1788-2015 compliance**: _required_, required accuracy mode: *accurate*.
        #[must_use]
        pub fn atan2(self, other: Self) -> Self {
            // The rectangle `other x self` (where x denotes cartesian product of intervals)
            // can be placed as following:
            //                         ^ self
            //                         |
            //    _____*         _____________         *_____
            //    |    |         |     |     |         |    |
            //    *____|         *___________*         |____*
            //                         |
            //                         |
            //   [-pi, pi]        [NaN,|Nan]
            //    ______         _____________         *____.
            //    |    |         |     |     |         |    |
            // ---|----|---------|-----|-----|---------|----|---> other
            //    |____|         |___________|         *____|
            //                         |
            //                         |
            //    *_____         *___________*         _____*
            //    |    |         |     |     |         |    |
            //    |____*         |___________|         *____|
            //                         |
            //
            // here (*) denotes the corners of a rectangle that have maximum and minimum argument
            if self.is_nai() || other.is_nai() {
                Self::NAI
            } else if self.is_empty() || other.is_empty() {
                Self::EMPTY
            } else if self.contains(0.) {
                if other.contains(0.) {
                    // Rectangle in the center
                    Self::NAI
                } else if other.0 > 0. {
                    // Rectangle on the right
                    Self(
                        crlibm::atan_rd(div_rd(self.0, other.0)),
                        crlibm::atan_ru(div_ru(self.1, other.0)),
                    )
                } else {
                    // Rectangle on the left
                    #[expect(clippy::float_arithmetic, reason = "negation is exact")]
                    Self(-Self::PI.1, Self::PI.1)
                }
            } else if self.0 > 0. {
                if other.contains(0.) {
                    // Rectangle on the top
                    Self(
                        crlibm::atan_rd(div_rd(self.0, other.1)),
                        sub_ru(Self::PI.1, crlibm::atan_rd(div_ru(self.0, other.0))),
                    )
                } else if other.0 > 0. {
                    // Rectangle on the top-right
                    Self(
                        crlibm::atan_rd(div_rd(self.0, other.1)),
                        crlibm::atan_ru(div_ru(self.1, other.0)),
                    )
                } else {
                    // Rectangle on the top-left
                    Self(
                        sub_rd(Self::PI.0, crlibm::atan_ru(div_ru(self.1, other.1))),
                        sub_ru(Self::PI.1, crlibm::atan_rd(div_ru(self.0, other.0))),
                    )
                }
            } else {
                if other.contains(0.) {
                    // Rectangle on the bottom
                    Self(
                        sub_rd(Self::PI.0, crlibm::atan_ru(div_ru(self.1, other.0))),
                        crlibm::atan_ru(div_ru(self.1, other.1)),
                    )
                } else if other.0 > 0. {
                    // Rectangle on the bottom-right
                    Self(
                        crlibm::atan_rd(div_rd(self.0, other.0)),
                        crlibm::atan_ru(div_ru(self.1, other.1)),
                    )
                } else {
                    // Rectangle on the bottom-left
                    Self(
                        sub_rd(Self::PI.0, crlibm::atan_ru(div_ru(self.1, other.0))),
                        sub_ru(Self::PI.1, crlibm::atan_rd(div_ru(self.0, other.1))),
                    )
                }
            }
        }
        /// Returns argument range of vectors with coordinates `x ∈ other` and `y ∈ self` divided by `π`.
        ///
        /// **Accuracy**: *unknown*.
        ///
        /// **IEEE 1788-2015 compliance**: _recommended_, required accuracy mode: *accurate*.
        #[must_use]
        pub fn atan2_pi(self, other: Self) -> Self {
            // The rectangle `other x self` (where x denotes cartesian product of intervals)
            // can be placed as following:
            //                         ^ self
            //                         |
            //    _____*         _____________         *_____
            //    |    |         |     |     |         |    |
            //    *____|         *___________*         |____*
            //                         |
            //                         |
            //   [-pi, pi]        [NaN,|Nan]
            //    ______         _____________         *____.
            //    |    |         |     |     |         |    |
            // ---|----|---------|-----|-----|---------|----|---> other
            //    |____|         |___________|         *____|
            //                         |
            //                         |
            //    *_____         *___________*         _____*
            //    |    |         |     |     |         |    |
            //    |____*         |___________|         *____|
            //                         |
            //
            // here (*) denotes the corners of a rectangle that have maximum and minimum argument
            if self.is_nai() || other.is_nai() {
                Self::NAI
            } else if self.is_empty() || other.is_empty() {
                Self::EMPTY
            } else if self.contains(0.) {
                if other.contains(0.) {
                    // Rectangle in the center
                    Self::NAI
                } else if other.0 > 0. {
                    // Rectangle on the right
                    Self(
                        crlibm::atanpi_rd(div_rd(self.0, other.0)),
                        crlibm::atanpi_ru(div_ru(self.1, other.0)),
                    )
                } else {
                    // Rectangle on the left
                    Self(-1., 1.)
                }
            } else if self.0 > 0. {
                if other.contains(0.) {
                    // Rectangle on the top
                    Self(
                        crlibm::atanpi_rd(div_rd(self.0, other.1)),
                        sub_ru(1., crlibm::atanpi_rd(div_ru(self.0, other.0))),
                    )
                } else if other.0 > 0. {
                    // Rectangle on the top-right
                    Self(
                        crlibm::atanpi_rd(div_rd(self.0, other.1)),
                        crlibm::atanpi_ru(div_ru(self.1, other.0)),
                    )
                } else {
                    // Rectangle on the top-left
                    Self(
                        sub_rd(1., crlibm::atanpi_ru(div_ru(self.1, other.1))),
                        sub_ru(1., crlibm::atanpi_rd(div_ru(self.0, other.0))),
                    )
                }
            } else {
                if other.contains(0.) {
                    // Rectangle on the bottom
                    Self(
                        sub_rd(1., crlibm::atanpi_ru(div_ru(self.1, other.0))),
                        crlibm::atanpi_ru(div_ru(self.1, other.1)),
                    )
                } else if other.0 > 0. {
                    // Rectangle on the bottom-right
                    Self(
                        crlibm::atanpi_rd(div_rd(self.0, other.0)),
                        crlibm::atanpi_ru(div_ru(self.1, other.1)),
                    )
                } else {
                    // Rectangle on the bottom-left
                    Self(
                        sub_rd(1., crlibm::atanpi_ru(div_ru(self.1, other.0))),
                        sub_ru(1., crlibm::atanpi_rd(div_ru(self.0, other.1))),
                    )
                }
            }
        }
        /// Returns hyperbolic sine of `self`.
        #[must_use]
        pub fn sinh(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(crlibm::sinh_rd(self.0), crlibm::sinh_ru(self.1))
            }
        }
        /// Returns hyperbolic cosine of `self`.
        #[must_use]
        pub fn cosh(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 >= 0. {
                Self(crlibm::cosh_rd(self.0), crlibm::cosh_ru(self.1))
            } else if self.1 <= 0. {
                Self(crlibm::cosh_rd(self.1), crlibm::cosh_ru(self.0))
            } else {
                let x = f64::max(self.0.abs(), self.1.abs());
                Self(1., crlibm::cosh_ru(x))
            }
        }
        /// Returns hyperbolic tangent of `self`.
        #[must_use]
        pub fn tanh(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(
                    div_rd(crlibm::sinh_rd(self.0), crlibm::cosh_ru(self.0)),
                    div_ru(crlibm::sinh_ru(self.0), crlibm::cosh_rd(self.0)),
                )
            }
        }
        /// Returns inverse hyperbolic sine of `self`.
        #[must_use]
        pub fn asinh(self) -> Self {
            if self.is_empty() {
                self
            } else {
                // asinh(x) = ln(x + sqrt(x^2 + 1)), where each subexpression is monotone in x
                Self(
                    crlibm::ln_rd(add_rd(self.0, sqrt_rd(add_rd(mul_rd(self.0, self.0), 1.)))),
                    crlibm::ln_ru(add_ru(self.1, sqrt_ru(add_ru(mul_ru(self.1, self.1), 1.)))),
                )
            }
        }
        /// Returns inverse hyperbolilc cosine of `self`.
        #[must_use]
        pub fn acosh(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 < 1. {
                Self::NAI
            } else {
                // acosh(x) = ln(x + sqrt(x^2 - 1)), where each subexpression is monotone in x >= 1
                Self(
                    crlibm::ln_rd(add_rd(self.0, sqrt_rd(sub_rd(mul_rd(self.0, self.0), 1.)))),
                    crlibm::ln_ru(add_ru(self.1, sqrt_ru(sub_ru(mul_ru(self.1, self.1), 1.)))),
                )
            }
        }
        #[must_use]
        pub fn atanh(self) -> Self {
            if self.is_empty() {
                self
            } else if self.0 < -1. || self.1 > 1. {
                Self::NAI
            } else {
                // atanh(x) = 1/2 ln((1 + x)/(1 - x)), where each subexpression is monotone for x in [-1, 1]
                Self(
                    mul_rd(
                        0.5,
                        crlibm::ln_rd(div_rd(add_rd(1., self.0), sub_ru(1., self.0))),
                    ),
                    mul_ru(
                        0.5,
                        crlibm::ln_ru(div_ru(add_ru(1., self.1), sub_rd(1., self.1))),
                    ),
                )
            }
        }
    }
}

pub(crate) mod integer_functions {
    use crate::Interval;

    /// Returns sign of the input, returning `1.` for positive values, `-1.` for negative values, and `+0.` for zero values `+0.` and `-0.`.
    #[must_use]
    pub(crate) const fn sign_f64(x: f64) -> f64 {
        #![allow(clippy::as_conversions, reason = "bool to bitmask coversion is needed")]

        #[expect(clippy::eq_op, reason = "nan detection")]
        #[expect(clippy::float_cmp, reason = "nan detection")]
        let is_nan = x != x;
        let is_non_zero = x != 0.;

        let bits = x.to_bits();

        // 0b0000..0000 when x == 0
        // 0b1111..1111 when x != 0
        let non_zero_mask = (is_non_zero as u64).wrapping_neg();

        // 0b0000..0000 when x is not nan
        // 0b1111..1111 when x is nan
        let nan_mask = (is_nan as u64).wrapping_neg();

        let sign = (1_f64.to_bits()) & non_zero_mask | (bits & f64::SIGN_MASK);

        f64::from_bits((bits & nan_mask) | sign)
    }

    impl Interval {
        /// Returns the tightest interval containing the signum of f64 values in `self`. Note: as in `f64::signum`, `+0` and `-0` values have signs `+1` and `-1`, not `0`.
        ///
        /// # Explicit formula
        ///
        /// Since `f64::signum` is monothonic, `[a, b].signum() = [a.signum(), b.signum()]`.
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns exact values.
        ///
        /// # Examples
        ///
        /// ```
        /// use ivar::iv;
        ///
        /// assert_eq!(iv![100.].signum(), iv![1.]);
        /// assert_eq!(iv![10., 20.].signum(), iv![1.]);
        /// assert_eq!(iv![-100.].signum(), iv![-1.]);
        /// assert_eq!(iv![-20., -10.].signum(), iv![-1.]);
        ///
        /// assert_eq!(iv![-10., 10.].signum(), iv![-1., 1.]);
        ///
        /// assert_eq!(iv!().signum(), iv!()); // signum of empty is empty
        ///
        /// // zero endpoints
        /// assert_eq!(iv![0., 0.].signum(), iv![1., 1.]);
        /// assert_eq!(iv![-0., -0.].signum(), iv![-1., -1.]);
        /// assert_eq!(iv![-0., 0.].signum(), iv![-1., 1.]);
        ///
        /// assert_eq!(iv![0., 123.456].signum(), iv![1.]);
        /// assert_eq!((-iv![0., 123.456]).signum(), iv![-1.]);
        /// assert_eq!(iv![-123.456, -0.].signum(), iv![-1.]);
        ///
        /// // infinite intervals
        /// assert_eq!(iv![..].signum(), iv![-1., 1.]);
        /// assert_eq!(iv![0. ..].signum(), iv![1.]);
        /// assert_eq!(iv![.. -0.].signum(), iv![-1.]);
        /// ```
        ///
        /// # Conformance to the standard
        ///
        /// This function is not a part of IEEE 1788-2015.
        ///
        #[must_use]
        pub const fn signum(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(self.0.signum(), self.1.signum())
            }
        }

        /// Returns the tightest interval containing the sign of values in `self`.
        ///
        /// # Explicit formula
        ///
        /// Since sign is monothonic function, `sign([a, b]) = [sign(a), sign(b)]`
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns exact values.
        ///
        /// # Examples
        ///
        /// ```
        /// use ivar::iv;
        ///
        /// assert_eq!(iv![100.].sign(), iv![1.]);
        /// assert_eq!(iv![10., 20.].sign(), iv![1.]);
        /// assert_eq!(iv![-100.].sign(), iv![-1.]);
        /// assert_eq!(iv![-20., -10.].sign(), iv![-1.]);
        ///
        /// assert_eq!(iv![-10., 10.].sign(), iv![-1., 1.]);
        ///
        /// assert_eq!(iv!().sign(), iv!()); // sign of empty is empty
        ///
        /// // zero endpoints
        /// assert_eq!(iv![0., 0.].sign(), iv![0., 0.]);
        /// assert_eq!(iv![-0., -0.].sign(), iv![0., 0.]);
        /// assert_eq!(iv![-0., 0.].sign(), iv![0., 0.]);
        ///
        /// assert_eq!(iv![0., 123.456].sign(), iv![0., 1.]);
        /// assert_eq!((-iv![0., 123.456]).sign(), iv![-1., 0.]);
        /// assert_eq!(iv![-123.456, -0.].sign(), iv![-1., 0.]);
        ///
        /// // infinite intervals
        /// assert_eq!(iv![..].sign(), iv![-1., 1.]);
        /// assert_eq!(iv![0. ..].sign(), iv![0., 1.]);
        /// assert_eq!(iv![.. -0.].sign(), iv![-1., 0.]);
        /// ```
        ///
        /// # Relation to the standard
        ///
        /// This function is required by the standard with _tightest_ accuracy mode.
        ///
        #[must_use]
        pub const fn sign(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(sign_f64(self.0), sign_f64(self.1))
            }
        }

        /// Returns the tightest interval containing the ceil of values in `self`.
        ///
        /// # Explicit formula
        ///
        /// Since ceil is monothonic function, `ceil([a, b]) = [ceil(a), ceil(b)]`
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns exact values.
        ///
        /// # Relation to the standard
        ///
        /// This function is required by the standard with _tightest_ accuracy mode.
        ///
        #[must_use]
        pub const fn ceil(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(self.0.ceil(), self.1.ceil())
            }
        }
        /// Returns the tightest interval containing the floor of values in `self`.
        ///
        /// # Explicit formula
        ///
        /// Since floor is monothonic function, `floor([a, b]) = [floor(a), floor(b)]`
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns exact values.
        ///
        /// # Relation to the standard
        ///
        /// This function is required by the standard with _tightest_ accuracy mode.
        ///
        #[must_use]
        pub const fn floor(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(self.0.floor(), self.1.floor())
            }
        }
        /// Returns the tightest interval containing the trunc of values in `self`.
        ///
        /// # Explicit formula
        ///
        /// Since trunc is monothonic function, `trunc([a, b]) = [trunc(a), trunc(b)]`
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns exact values.
        ///
        /// # Relation to the standard
        ///
        /// This function is required by the standard with _tightest_ accuracy mode.
        ///
        #[must_use]
        pub const fn trunc(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(self.0.trunc(), self.1.trunc())
            }
        }
        /// Returns the tightest interval containing all rounded values of `self`.
        ///
        /// # Explicit formula
        ///
        /// Since round is monothonic function, `round([a, b]) = [round(a), round(b)]`
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns exact values.
        ///
        /// # Relation to the standard
        ///
        /// This function is required by the standard with _tightest_ accuracy mode.
        /// Name in the standard: `roundTiesToAway`. Defined at: Table 9.1
        ///
        #[must_use]
        pub const fn round(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(self.0.round(), self.1.round())
            }
        }
        ///
        /// # Relation to the standard
        ///
        /// This function is required by the standard with _tightest_ accuracy mode.
        ///
        #[must_use]
        pub const fn round_ties_even(self) -> Self {
            if self.is_empty() {
                self
            } else {
                Self(self.0.round_ties_even(), self.1.round_ties_even())
            }
        }
        /// Returns the tightest interval containing fractional parts of values in `self`.
        ///
        /// # Accuracy
        ///
        /// *Accuracy mode*: *tightest*. This function always returns precise results.
        ///
        /// ```
        /// use ivar::Interval;
        ///
        /// assert_eq!(Interval::new(100., 100.).fract(), Interval::new(0., 0.));
        /// assert_eq!(Interval::new(-100., -100.).fract(), Interval::new(0., 0.));
        /// assert_eq!(Interval::new(3.5, 3.5).fract(), Interval::new(0.5, 0.5));
        /// assert_eq!(Interval::new(-3.5, -3.5).fract(), Interval::new(-0.5, -0.5));
        /// assert_eq!(Interval::new(3.25, 3.75).fract(), Interval::new(0.25, 0.75));
        /// assert_eq!(Interval::new(-3.75, -3.25).fract(), Interval::new(-0.75, -0.25));
        /// assert_eq!(Interval::new(-0.4, 0.6).fract(), Interval::new(-0.4, 0.6));
        /// assert_eq!(Interval::new(2.5, 3.25).fract(), Interval::new(0., 1.));
        /// assert_eq!(Interval::new(-3.25, -2.5).fract(), Interval::new(-1., 0.));
        /// assert_eq!(Interval::new(-3.25, 2.5).fract(), Interval::new(-1., 1.));
        /// assert_eq!(Interval::new(-0.25, 2.5).fract(), Interval::new(-0.25, 1.));
        /// assert_eq!(Interval::new(-3.25, 0.5).fract(), Interval::new(-1., 0.5));
        /// ```
        ///
        /// # Relation to the standard
        ///
        /// This function is not a part of the standard.
        ///
        #[must_use]
        pub const fn fract(self) -> Self {
            let trunc = self.trunc();
            #[expect(clippy::float_cmp, reason = "comparison of integer values")]
            if trunc.1 == trunc.0 {
                self.cancel_add(self.trunc())
            } else if trunc.0 >= 0. {
                Self(f64::min(0., self.0), 1.)
            } else if trunc.1 <= 0. {
                Self(-1., f64::max(0., self.1))
            } else {
                Self(-1., 1.)
            }
        }
    }
}

pub(crate) mod abs_max_functions {
    impl crate::Interval {
        #[must_use]
        /// Return the absolute value of `self`
        pub const fn abs(self) -> Self {
            if self.is_empty() {
                self
            } else if self.contains(0.) {
                Self(0., f64::max(self.0.abs(), self.1.abs()))
            } else if self.0 > 0. {
                self
            } else {
                self.neg()
            }
        }
        /// Returns the minimum of two intervals, ignoring `NAI`.
        #[must_use]
        pub const fn min(self, other: Self) -> Self {
            Self(f64::min(self.0, other.0), f64::min(self.1, other.1))
        }
        /// Returns the maximum of two intervals, ignoring `NAI`.
        #[must_use]
        pub const fn max(self, other: Self) -> Self {
            Self(f64::max(self.0, other.0), f64::max(self.1, other.1))
        }
        /// Returns
        #[must_use]
        pub const fn abs_sub(self, other: Self) -> Self {
            let sub = self.sub(other);
            Self(sub.0.max(0.), sub.1.max(0.))
        }
    }
}
