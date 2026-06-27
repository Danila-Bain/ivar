/// Implementation of [`cancel_add`] and [`cancel_sub`] functions
mod cancellative_addition_and_subtraction {
    use crate::rounded_arithmetic::{add_rd, add_ru, sub_rd, sub_ru};

    impl crate::Interval {
        /// For two intervals `x`, `y` returns the interval `z` such that `x = z + y`.
        /// the formula is [z.0, z.1] = [x.0 - y.0, x.1 - y.1]. Note that this is similar to subtraction but the order of bounds of `y` are not swapped.
        #[must_use]
        pub const fn cancel_add(self, other: Self) -> Self {
            Self::new(sub_rd(self.0, other.0), sub_ru(self.1, other.1))
        }
        /// For two intervals `x`, `y` returns the interval `z` such that `x = z - y`.
        /// the formula is [z.0, z.1] = [x.0 + y.1, x.1 + y.0]. Note that this is similar to addition but the order of bounds of `y` are not swapped.
        #[must_use]
        pub const fn cancel_sub(self, other: Self) -> Self {
            Self::new(add_rd(self.0, other.1), add_ru(self.1, other.0))
        }
    }
}

/// Implementation of unary reverse elementary functions
mod from_unary_functions {
    use crate::{
        Interval,
        rounded_arithmetic::{add_rd, div_rd, div_ru, sqrt_rd, sqrt_ru, sub_ru},
    };

    impl Interval {
        /// Return the interval containing all values `x ∈ domain` such that `x² ∈ self`.
        ///
        /// This function is closely related to `sqrt(self)` but has distinctions, demonstrated by the following examples:
        /// - `sqr_rev([1, 4], ℝ) = hull([-2, -1] ∪ [1, 2]) = [-2, 2]`,
        /// - `sqr_rev([1, 4], [0, +∞)) = hull(([-2, -1] ∪ [1, 2]) ∩ (0, +∞)) = [1, 2]`,
        /// - `sqr_rev([1, 4], (-∞, 0]) = hull(([-2, -1] ∪ [1, 2]) ∩ (-∞, 0)) = [-2, -1]`,
        ///
        /// while `sqrt([1, 4]) = [1, 2]`.
        ///
        /// - `sqr_rev([-1, 4], [0, +∞)) = [0, 2]`,
        /// - `sqr_rev([-1, 4], (-∞, 0]) = [-2, 0]`,
        ///
        /// while `sqrt([-1, 4])` is not defined and evaluated to `nai`.
        ///
        /// # Examples
        /// ```
        /// use ivar::iv;
        ///
        /// assert_eq!(iv![1., 4.].sqr_rev(iv!(0. ..)), iv![1., 2.]); // principal square root
        /// assert_eq!(iv![1., 4.].sqr_rev(iv!(.. 0.)), iv![-2., -1.]); // minus principal square root
        /// assert_eq!(iv![1., 4.].sqr_rev(iv!(..)), iv![-2., 2.]); // hull of multivalued square root
        ///
        /// assert_eq!(iv![1., 16.].sqr_rev(iv!(2. ..)), iv![2., 4.]); // filtered principal square root
        ///
        /// // unobtainable values are ignored
        /// assert_eq!(iv![-100., 4.].sqr_rev(iv!(0. ..)), iv![0., 2.]);
        /// assert_eq!(iv![-100., -1.].sqr_rev(iv!(..)), iv!(empty));
        /// assert_eq!(iv![-100., 0.].sqr_rev(iv!(..)), iv!(0.));
        /// ```
        #[must_use]
        pub fn sqr_rev(self, domain: Self) -> Self {
            if self.is_nai() || domain.is_nai() {
                Self::NAI
            } else if self.is_empty() || domain.is_empty() || self.1 < 0. {
                Self::EMPTY
            } else if self.0 <= 0. {
                let sqrt = sqrt_ru(self.1);
                #[expect(clippy::float_arithmetic, reason = "negation is exact")]
                Self(-sqrt, sqrt).inter(domain)
            } else {
                // (self.0 > 0) : interval is positive
                let sqrt_sup = sqrt_ru(self.1);
                let sqrt_inf = sqrt_rd(self.0);
                Self::hull(
                    #[expect(clippy::float_arithmetic, reason = "negation is exact")]
                    Self(-sqrt_sup, -sqrt_inf).inter(domain),
                    Self(sqrt_inf, sqrt_sup).inter(domain),
                )
            }
        }

        /// Returns an interval containing all $x \in \texttt{domain}$ such that $|x| \in \texttt{self}$.
        #[must_use]
        pub const fn abs_rev(self, domain: Self) -> Self {
            if self.is_nai() || domain.is_nai() {
                Self::NAI
            } else if self.is_empty() || domain.is_empty() || self.1 < 0. {
                Self::EMPTY
            } else if self.0 <= 0. {
                #[expect(clippy::float_arithmetic, reason = "negation is exact")]
                Self(-self.1, self.1).inter(domain)
            } else {
                // (self.0 > 0) : interval is positive
                Self::hull(Self::inter(self, domain), Self::inter(self.neg(), domain))
            }
        }

        /// Returns an interval containing all `x ∈ domain` such that `xⁿ ∈ self`.
        ///
        /// This is similar to `ⁿ√self` except it is treated as multivlued function ignoring invalid inputs.
        ///
        /// ```
        /// use ivar::{iv, Interval};
        ///
        /// assert_eq!(iv!(1., 4.).powi_rev(2, Interval::POSITIVE), iv!(1., 2.));
        /// assert_eq!(iv!(1., 4.).powi_rev(-2, Interval::POSITIVE), iv!(0.5_f64.next_down(), 1.));
        /// assert_eq!(iv!(-1., 8.).powi_rev(3, Interval::ENTIRE), iv!(-1., 2.));
        /// assert_eq!(iv!(-1., 16.).powi_rev(4, Interval::ENTIRE), iv!(-2., 2.).next_out());
        /// assert_eq!(iv!(-1., 16.).powi_rev(4, Interval::POSITIVE), iv!(0., 2f64.next_up()));
        /// ```
        #[must_use]
        pub fn powi_rev(self, n: i32, domain: Self) -> Self {
            #![expect(clippy::shadow_same, reason = "match special integer values")]
            match n {
                0 if self.contains(1.) => domain,
                0 => Self::EMPTY,
                1 => self.inter(domain),
                -1 => self.recip().inter(domain),
                2 => self.sqr_rev(domain),      // better precision
                3 => self.cbrt().inter(domain), // better precision
                n if n & 1 == 0 => {
                    let pow = self
                        .inter(Self::POSITIVE)
                        .pow(Self::new_singleton(f64::from(n)).recip());
                    Self::hull(pow.inter(domain), pow.neg().inter(domain))
                }
                n => {
                    let pos = self
                        .inter(Self::POSITIVE)
                        .pow(Self::new_singleton(f64::from(n)).recip());
                    let neg = self
                        .neg()
                        .inter(Self::POSITIVE)
                        .pow(Self::new_singleton(f64::from(n)).recip())
                        .neg();

                    Self::hull(pos.inter(domain), neg.inter(domain))
                }
            }
        }
        /// Returns an interval containing all `x ∈ domain` such that `sin(x) ∈ self`.
        ///
        /// This is similar to `asin(self)` except it is treated as multivlued function ignoring invalid inputs.
        ///
        ///
        /// # Examples
        /// ```
        /// use ivar::{iv, Interval};
        ///
        /// // `sin(x) ∈ [-1, 1]` everywhere
        /// assert_eq!(iv![-1., 1.].sin_rev(iv![-12., 12.]), iv![-12., 12.]);
        /// assert_eq!(iv![-1., 1.].sin_rev(iv![1., 1.1]), iv![1., 1.1]);
        ///
        /// // `sin(x) ∈ [0, 1]` for all `x ∈ [0, π]`
        /// assert_eq!(iv![0., 1.].sin_rev(iv![-3.14, 6.28]), iv![0., Interval::PI.sup()]);
        /// assert_eq!(iv![0., 1.].sin_rev(iv![3.15, 9.45]), iv![2. * Interval::PI.inf(), 3. * Interval::PI.sup()]);
        /// ```
        #[must_use]
        pub fn sin_rev(self, domain: Self) -> Self {
            //                                                  |                                                  //
            //                       _---_                      |     _---_                            _---_       //
            //                     #‾     ‾#               s|#  |   #‾     ‾#                        #‾     ‾#     //
            //                    #         #              e|#  |  #         #                      #         #    //
            //                   #           #             l|#  | #           #                    #          :#   //
            //        -5π       #     -3π     #        -π  f|#  |#      π      #        3π        #     5π    : #  //
            // -------------(-2π)------------(-π)---------------0---------------π----------------2π--------------- //
            // ‾       2      ‾        2        ‾       2      ‾|       2        ‾       2   :  ‾ :      2    :    //
            //  -            -                   -            - |                 -          : -  :           :    //
            //   -          -                     -          -  |                  -         :-   :           :    //
            //     -       -                        -       -   |                    -       -    :           :    //
            //       ‾---‾                            ‾---‾     |                      ‾---‾ :    :           :    //
            //                                                  |                            :    :           :    //
            //                                                  |                            |-----domain-----|    //
            //                                                  |                                 :           :    //
            //                                                  |                                 :           :    //
            //                                                  |                                 |--sin-rev--|    //
            if self.is_nai() || domain.is_nai() {
                Self::NAI
            } else if self.is_empty() || domain.is_empty() {
                Self::EMPTY
            } else {
                let asin = Self(
                    if self.0 <= -1. {
                        #[expect(clippy::float_arithmetic, reason = "negation is exact")]
                        -Self::FRAC_PI_2.1
                    } else {
                        crlibm::asin_rd(self.0)
                    },
                    if self.1 >= 1. {
                        Self::FRAC_PI_2.1
                    } else {
                        crlibm::asin_ru(self.1)
                    },
                );
                // Result will be the intersection of `domain` with the union of the set
                // ` ... ∪ (asin - 4π) ∪ (asin - 2π) ∪ (asin) ∪ (asin + 2π) ∪ ...`
                // and
                // ` ... ∪ (-3π - asin) ∪ (-π - asin) ∪ (π - asin) ∪ (3π - asin) ∪ ...`

                // smallest `n` such that domain intersects `[n*π - π/2, n*π + π/2]`
                // (domain.0 + π/2)/π >= n_inf  <==> domain.0 >= n_inf*π - π/2
                let n_inf = add_rd(div_rd(domain.0, Self::PI.0), 0.5).floor();
                // largest `n` such that domain intersects `[n*π - π/2, n*π + π/2]`
                // (domain.1 - π/2)/π <= n_sup  <==> domain.1 <= n_sup*π + π/2
                let n_sup = sub_ru(div_ru(domain.1, Self::PI.0), 0.5).ceil();

                #[expect(
                    clippy::float_arithmetic,
                    clippy::modulo_arithmetic,
                    reason = "n_inf.abs() is non-negative integer, so modulo 2 is exact and has value 0. or 1."
                )]
                let n_inf_parity = n_inf.abs() % 2.;
                #[expect(
                    clippy::float_arithmetic,
                    clippy::modulo_arithmetic,
                    reason = "n_sup.abs() is non-negative integer, so modulo 2 is exact and has value 0. or 1."
                )]
                let n_sup_parity = n_sup.abs() % 2.;

                let n_inf_iv = Self(n_inf, n_inf);
                let n_sup_iv = Self(n_sup, n_sup);

                let right = if n_sup_parity == 0. {
                    Self::hull(
                        domain.inter(n_sup_iv * Self::PI + asin),
                        domain.inter((n_sup_iv - 1.) * Self::PI - asin),
                    )
                } else {
                    Self::hull(
                        domain.inter(n_sup_iv * Self::PI - asin),
                        domain.inter((n_sup_iv - 1.) * Self::PI + asin),
                    )
                };

                let left = if n_inf_parity == 0. {
                    Self::hull(
                        domain.inter(n_inf_iv * Self::PI + asin),
                        domain.inter((n_inf_iv + 1.) * Self::PI - asin),
                    )
                } else {
                    Self::hull(
                        domain.inter(n_inf_iv * Self::PI - asin),
                        domain.inter((n_inf_iv + 1.) * Self::PI + asin),
                    )
                };

                left.hull(right)
            }
        }
        /// Returns an interval containing all `x ∈ domain` such that `sin(x) ∈ self`.
        ///
        /// This is similar to `acos(self)` except it is treated as multivlued function ignoring invalid inputs.
        ///
        /// # Examples
        /// ```
        /// use ivar::{iv, Interval};
        ///
        /// // `cos(x) ∈ [-1, 1]` everywhere
        /// assert_eq!(iv![-1., 1.].cos_rev(iv![-12., 12.]), iv![-12., 12.]);
        /// assert_eq!(iv![-1., 1.].cos_rev(iv![1., 1.1]), iv![1., 1.1]);
        ///
        /// // `cos(x) ∈ [0, 1]` for all `x ∈ [-π/2, π/2]`
        /// assert_eq!(
        ///     iv![0., 1.].cos_rev(iv![-3.14, 3.14]),
        ///     iv![-Interval::FRAC_PI_2.sup(), Interval::FRAC_PI_2.sup()],
        /// );
        /// assert_eq!(
        ///     iv![0., 1.].cos_rev(iv![3.15, 9.45]),
        ///     iv![
        ///         3.*Interval::FRAC_PI_2.inf(),
        ///         5.*Interval::FRAC_PI_2.sup(),
        ///     ].next_out(), // widening due to rounding errors
        /// );
        /// ```
        #[must_use]
        pub fn cos_rev(self, domain: Self) -> Self {
            //                                                  |                                                   //
            //               _---_                            _---_                            _---_                //
            //             -‾     ‾-                        -‾  |  ‾-                        -‾     ‾-              //
            //            -         -                      -    |    -                      -         -             //
            //           -           -                    -     |     -                    -           -            //
            //          _             _                  _      |      _                  _             _           //
            // -------------------------------------------------0-------------------------------------------------- //
            //        ‾                 ‾              ‾        |        ‾              ‾                 ‾         //
            //       -                   -            -         |         -            -                   -        //
            //      -                     -          -          |          -          -                     -       //
            //     -                        -       -           |            -       -                        -     //
            // --‾                            ‾---‾             |              ‾---‾                            ‾-- //
            //                                                  |                                                   //
            if self.is_nai() || domain.is_nai() {
                Self::NAI
            } else if self.is_empty() || domain.is_empty() {
                Self::EMPTY
            } else {
                let acos = Self(
                    if self.1 >= 1. {
                        0.
                    } else {
                        crlibm::acos_rd(self.1)
                    },
                    if self.0 <= -1. {
                        Self::PI.1
                    } else {
                        crlibm::acos_ru(self.0)
                    },
                );
                // Result will be the intersection of `domain` with the union of the set
                // ` ... ∪ (- 4π + acos) ∪ (- 2π + acos) ∪ (acos) ∪ (2π + acos) ∪ ...`
                // and
                // ` ... ∪ (- 4π - acos) ∪ (- 2π - acos) ∪ (acos) ∪ (2π - acos) ∪ ...`

                // smallest `n` such that domain intersects `[n*π, n*π + π]`
                // domain.0/π >= n_l  <==> domain.0 >= n_l*π
                let n_inf = div_rd(domain.0, Self::PI.0).floor();
                // largest `n` such that domain intersects `[n*π, n*π + π]`
                // domain.1/π - 1 <= n_r  <==> domain.1 <= n_r*π + π
                let n_sup = sub_ru(div_ru(domain.1, Self::PI.0), 1.).ceil();

                #[expect(
                    clippy::float_arithmetic,
                    clippy::modulo_arithmetic,
                    reason = "n_inf.abs() is non-negative integer, so modulo 2 is exact and has value 0. or 1."
                )]
                let n_inf_parity = n_inf.abs() % 2.;
                #[expect(
                    clippy::float_arithmetic,
                    clippy::modulo_arithmetic,
                    reason = "n_sup.abs() is non-negative integer, so modulo 2 is exact and has value 0. or 1."
                )]
                let n_sup_parity = n_sup.abs() % 2.;

                let n_inf_iv = Self(n_inf, n_inf);
                let n_sup_iv = Self(n_sup, n_sup);

                let right = if n_sup_parity == 0. {
                    Self::hull(
                        domain.inter(n_sup_iv * Self::PI + acos),
                        domain.inter(n_sup_iv * Self::PI - acos),
                    )
                } else {
                    Self::hull(
                        domain.inter((n_sup_iv + 1.) * Self::PI - acos),
                        domain.inter((n_sup_iv - 1.) * Self::PI + acos),
                    )
                };

                let left = if n_inf_parity == 0. {
                    Self::hull(
                        domain.inter(n_inf_iv * Self::PI + acos),
                        domain.inter(n_inf_iv * Self::PI - acos),
                    )
                } else {
                    Self::hull(
                        domain.inter((n_inf_iv + 1.) * Self::PI - acos),
                        domain.inter((n_inf_iv + 1.) * Self::PI + acos),
                    )
                };

                left.hull(right)
            }
        }
        /// Returns an interval containing all `x ∈ domain` such that `tan(x) ∈ self`.
        ///
        /// This is similar to `atan(self)` except it is treated as multivlued function ignoring invalid inputs.
        #[must_use]
        pub fn tan_rev(self, domain: Self) -> Self {
            //     ↑ tan(x)              ● |                     ● |                     ● |
            //     |                       |                       |    ●                  |
            //     |                       |                       |    | s                |
            //     |                    ●  |                    ●  |    |               ●  |
            //     3                       |                       |    | e                |
            //     |                   ●   |                   ●   |    |              ●   |
            //     |                       |                       |    | l                |
            //     2                  ●    |                  ●    |    |             ●    |
            //     |                       |                       |    | f                |
            //     |                 ●     |                 ●     |    ●            ●     |
            //     1                ●      |                ●      |                ●      |
            //     |               ●       |               ●       |               ●       |
            //     |             ●         |             ●         |             ●         |
            //     +-----------●-----------|-----------●-----------|-----------●-----------|--> x
            //     |         ● -π         -π/2       ● 0          π/2        ● π         3π/2
            //     |       ●               |       ●               |       ●               |
            //    -1      ●                |      ●                |      ●                |
            //     |     ●                 |     ●                 |     ●                 |
            //     |                       |                       |                       |
            //    -2    ●                  |    ●                  |    ●                  |
            //     |                       |                       |                       |
            //     |   ●                   |   ●                   |   ●                   |
            //    -3                       |                       |                       |
            //     |  ●                    |  ●                    |  ●                    |
            //     |                       |                       |                       |
            //     |                       |                 ●--------domain--------●      |
            //     | ●                     | ●                     | ●                     |

            if self.is_nai() || domain.is_nai() {
                Self::NAI
            } else if self.is_empty() || domain.is_empty() {
                Self::EMPTY
            } else {
                let atan = self.atan();

                // largest `n` such that domain intersects `[n*π - π/2, n*π + π/2]`
                // (domain.1 - π/2)/π <= n_r  <==> domain.1 <= n_r*π + π/2
                let n_r = sub_ru(div_ru(domain.1, Self::PI.0), 0.5).ceil();

                // smallest `n` such that domain intersects `[n*π - π/2, n*π + π/2]`
                // (domain.0 + π/2)/π >= n_l  <==> domain.0 >= n_l*π - π/2
                let n_l = add_rd(div_rd(domain.0, Self::PI.0), 0.5).floor();

                let left = Self::hull(
                    domain.inter(n_l * Self::PI + atan),
                    domain.inter((Self(n_l, n_l) + 1.) * Self::PI + atan),
                );
                let right = Self::hull(
                    domain.inter(n_r * Self::PI + atan),
                    domain.inter((Self(n_r, n_r) - 1.) * Self::PI + atan),
                );
                Self::hull(left, right)
            }
        }

        /// Returns an interval containing all `x ∈ domain` such that `cosh(x) ∈ self`.
        ///
        /// This is similar to `acosh(self)` except it is treated as multivlued function ignoring invalid inputs.
        #[must_use]
        pub fn cosh_rev(self, domain: Self) -> Self {
            //     ↑ cosh(x)               |                       |
            //     |       ●               |               ●       |
            //     |                       |  ●                    |
            //     |                       |  | s                  |
            //     |        ●              |  |           ●        |
            //     |                       |  | e                  |
            //     |                       |  |                    |
            //     |          ●            |  | l       ●          |
            //     |                       |  |                    |
            //     |                       |  | f                  |
            //     |                       |  ●                    |
            //     |             ●         |         ●             |
            //     |                       |                       |
            //     |               ●       |       ●               |
            //     |                 ●     |     ●                 |
            //     1                       ●                       |
            //     |                       |                       |
            //     |                   ●--------domain--------●    |
            //     |                       |                       |
            //     |-----------------------0-----------------------|-> x
            //     |                       |                       |
            let acosh = self.inter(Self(1., f64::INFINITY)).acosh();
            Self::hull(acosh.inter(domain), acosh.neg().inter(domain))
        }
    }
}

/// Implementation of binary reverse elementary functions
mod from_binary_functions {
    use crate::{
        elementary_functions::basic_operations::min_max_4_pairs,
        rounded_arithmetic::{div_lossless, div_rd, div_ru, from_2f64_rd, from_2f64_ru},
    };

    impl crate::Interval {
        /// Returns an interval containing all `x ∈ domain` such that `x * rhs ∈ self`.
        #[must_use]
        pub fn mul_rev(self, rhs: Self, domain: Self) -> Self {
            if self.is_nai() || rhs.is_nai() || domain.is_nai() {
                Self::NAI
            } else if self.is_empty() || rhs.is_empty() || domain.is_empty() {
                Self::EMPTY
            } else if rhs.interior_contains(0.) {
                match self {
                    Self(a, _) if a > 0. => Self::hull(
                        Self(f64::NEG_INFINITY, div_ru(a, rhs.0)).inter(domain),
                        Self(div_rd(a, rhs.1), f64::INFINITY).inter(domain),
                    ),
                    Self(_, b) if b < 0. => Self::hull(
                        Self(f64::NEG_INFINITY, div_ru(b, rhs.1)).inter(domain),
                        Self(div_rd(b, rhs.0), f64::INFINITY).inter(domain),
                    ),
                    _ => domain,
                }
            } else if rhs.0 == 0. {
                if rhs.1 == 0. {
                    if self.contains(0.) {
                        domain
                    } else {
                        Self::EMPTY
                    }
                } else {
                    // rhs.1 > 0
                    match self {
                        Self(a, _) if a >= 0. => {
                            Self(div_rd(a, rhs.1), f64::INFINITY).inter(domain)
                        }
                        Self(_, b) if b <= 0. => {
                            Self(f64::NEG_INFINITY, div_ru(b, rhs.1)).inter(domain)
                        }
                        _ => domain,
                    }
                }
            } else if rhs.1 == 0. {
                // && rhs.0 < 0.
                match self {
                    Self(a, _) if a >= 0. => {
                        Self(f64::NEG_INFINITY, div_ru(a, rhs.0)).inter(domain)
                    }
                    Self(_, b) if b <= 0. => Self(div_rd(b, rhs.0), f64::INFINITY).inter(domain),
                    _ => domain,
                }
            } else {
                let divisions = [
                    div_lossless(self.0, rhs.0),
                    div_lossless(self.1, rhs.0),
                    div_lossless(self.0, rhs.1),
                    div_lossless(self.1, rhs.1),
                ];
                let (inf_pair, sup_pair) = min_max_4_pairs(divisions);
                Self(from_2f64_rd(inf_pair), from_2f64_ru(sup_pair)).inter(domain)
            }
        }
        /// Returns an interval containing all `x ∈ domain` for which there is `p ∈ power` such that `x^p ∈ self`.
        #[must_use]
        pub fn pow_rev1(self, power: Self, domain: Self) -> Self {
            // needs testing
            self.pow(power.recip()).inter(domain)
        }
        /// Returns an interval containing all `x ∈ domain` for which there is `b ∈ base` such that `b^x ∈ self`.
        #[must_use]
        pub fn pow_rev2(self, base: Self, domain: Self) -> Self {
            self.log(base).inter(domain)
        }
        /// Returns an interval containing all `y ∈ domain` such that `atan2(y, x) ∈ self`.
        #[must_use]
        pub fn atan2_rev1(self, x: Self, domain: Self) -> Self {
            //          |              /--self (angle)--⁄                //
            //          |             /              ⁄                //
            //          |            /|   |        ⁄                  //
            //     ●------------------|---|------⁄----                //
            //     |d   |          /  |   |    ⁄     |                //
            //     |o   |         /   |   |  ⁄       |                //
            //     |m   |        /    |   |⁄         | atan2_rev1     //
            //     |a   |       /     |  ⁄|          |                //
            //     |i   |      /      |⁄  |          |                //
            //     |n   |     /      ⁄|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾                //
            //     ●----|----/-----⁄  |   |                           //
            //          |   /    ⁄    |   |                           //
            //          |  /   ⁄      |   |                           //
            //          | /  ⁄        |   |                           //
            //          |/ ⁄          |   |                           //
            // ---------|-------------|---|-----=> other              //
            //          |             |   |                           //
            //          |             ●-x-●                           //
            (x * self.tan()).inter(domain)
        }
        /// Returns an interval containing all `x ∈ domain` such that `atan2(y, x) ∈ self`.
        #[must_use]
        pub fn atan2_rev2(self, y: Self, domain: Self) -> Self {
            (y / self.tan()).inter(domain)
        }

        /// Returns a pair of intervals representing the set of all `x` such that
        /// `x * y ∈ self` for some `y ∈ rhs`. This is the interval "reverse
        /// multiplication" or division operation, which may yield up to two
        /// disjoint components when `rhs` contains zero.
        ///
        /// If the corresponding set is empty, `(∅, ∅)` is returned.
        ///
        /// If the corresponding set is a single interval `I`, `(I, ∅)` is returned.
        ///
        /// If the corresponding set is a union of two intervals `I ∪ J` with `I < J`, then `(I, J)` is returned.
        ///
        /// This function is *required* by IEEE 1788-2015 (section 10.5.5.), where it is referred to as `mulRevToPair`.
        ///
        /// # Examples
        ///
        /// ```
        /// use ivar::interval as iv;
        ///
        /// // [2., 4.] / 2. = [1., 2.]
        /// assert_eq!(iv!(2., 4.).mul_rev_to_pair(iv!(2.)), (iv!(1., 2.), iv!())
        /// );
        ///
        /// // [2., 4.] / [1., 2.] = [1., 4.]
        /// assert_eq!(iv!(2., 4.).mul_rev_to_pair(iv!(1., 2.)), (iv!(1., 4.), iv!())
        /// );
        ///
        /// // [1., 2.] / [-1., 1.] = (-oo, -1.] U [1., +oo)
        /// assert_eq!(
        ///     iv!(1., 2.).mul_rev_to_pair(iv!(-1., 1.)),
        ///     (iv!(.. -1.), iv!(1. ..))
        /// );
        ///
        ///
        /// // [2., 4.] / [-2., 1.] = (-∞, -1.] ∪ [2., +∞)
        /// assert_eq!(
        ///     iv!(2., 4.).mul_rev_to_pair(iv!(-2., 1.)),
        ///     (iv!(.. -1.), iv!(2. ..))
        /// );
        ///
        ///
        /// // [-4., -2.] / [2., 4.] = [-2., -0.5]
        /// assert_eq!(iv!(-4., -2.).mul_rev_to_pair(iv!(2., 4.)), (iv!(-2., -0.5), iv!()));
        /// // [2., 4.] / [-4., -2.] = [-2., -0.5]
        /// assert_eq!(iv!(2., 4.).mul_rev_to_pair(iv!(-4., -2.)), (iv!(-2., -0.5), iv!()));
        /// // [-4., -2.] / [-4., -2.] = [0.5, 2.]
        /// assert_eq!(iv!(-4., -2.).mul_rev_to_pair(iv!(-4., -2.)), (iv!(0.5, 2.), iv!()));           
        /// // [1., 2.] / [0.] is empty
        /// assert_eq!(iv![1., 2.].mul_rev_to_pair(iv![0.]), (iv!(), iv!()));
        /// // [0., 2.] / [0.] is entrie
        /// assert_eq!(iv![0., 2.].mul_rev_to_pair(iv![0.]), (iv!(..), iv!()));
        /// // [0., 2.] / [0., 1.] is [0., +oo)
        /// assert_eq!(iv![0., 2.].mul_rev_to_pair(iv![0., 1.]), (iv!(0. ..), iv!()));
        /// // [-1., 1.] / [0., 1.] = (-oo, +oo)
        /// assert_eq!(iv![-1., 1.].mul_rev_to_pair(iv![0., 1.]), (iv!(..), iv!()));
        /// // [3., 4.] / [0., 1.] = [3., +oo)
        /// assert_eq!(iv![3., 4.].mul_rev_to_pair(iv![0., 1.]), (iv!(3. ..), iv!()));
        /// // [-3., -2.] / [0., 1.] = (-oo, -2.]
        /// assert_eq!(iv![-3., -2.].mul_rev_to_pair(iv![0., 1.]), (iv!(.. -2.), iv!()));
        /// ```
        #[must_use]
        pub const fn mul_rev_to_pair(self, rhs: Self) -> (Self, Self) {
            if self.is_nai() || rhs.is_nai() {
                (Self::NAI, Self::NAI)
            } else if self.is_empty() || rhs.is_empty() {
                (Self::EMPTY, Self::EMPTY)
            } else if rhs.interior_contains(0.) {
                match self {
                    Self(a, _) if a > 0. => (
                        Self(f64::NEG_INFINITY, div_ru(a, rhs.0)),
                        Self(div_rd(a, rhs.1), f64::INFINITY),
                    ),
                    Self(_, b) if b < 0. => (
                        Self(f64::NEG_INFINITY, div_ru(b, rhs.1)),
                        Self(div_rd(b, rhs.0), f64::INFINITY),
                    ),
                    _ => (Self::ENTIRE, Self::EMPTY),
                }
            } else if rhs.0 == 0. {
                if rhs.1 == 0. {
                    if self.contains(0.) {
                        (Self::ENTIRE, Self::EMPTY)
                    } else {
                        (Self::EMPTY, Self::EMPTY)
                    }
                } else {
                    // rhs.1 > 0
                    match self {
                        Self(a, _) if a >= 0. => {
                            (Self(div_rd(a, rhs.1), f64::INFINITY), Self::EMPTY)
                        }
                        Self(_, b) if b <= 0. => {
                            (Self(f64::NEG_INFINITY, div_ru(b, rhs.1)), Self::EMPTY)
                        }
                        _ => (Self::ENTIRE, Self::EMPTY),
                    }
                }
            } else if rhs.1 == 0. {
                // && rhs.0 < 0.
                match self {
                    Self(a, _) if a >= 0. => {
                        (Self(f64::NEG_INFINITY, div_ru(a, rhs.0)), Self::EMPTY)
                    }
                    Self(_, b) if b <= 0. => (Self(div_rd(b, rhs.0), f64::INFINITY), Self::EMPTY),
                    _ => (Self::ENTIRE, Self::EMPTY),
                }
            } else {
                let divisions = [
                    div_lossless(self.0, rhs.0),
                    div_lossless(self.1, rhs.0),
                    div_lossless(self.0, rhs.1),
                    div_lossless(self.1, rhs.1),
                ];
                let (inf_pair, sup_pair) = min_max_4_pairs(divisions);
                (
                    Self(from_2f64_rd(inf_pair), from_2f64_ru(sup_pair)),
                    Self::EMPTY,
                )
            }
        }
    }
}
