use crate::{Interval, iv};

/// Trivial `Interval` constants
impl Interval {
    /// Empty interval (`∅`)
    pub const EMPTY: Self = Self(f64::INFINITY, f64::NEG_INFINITY);
    /// Entire interval: `(-∞, ∞)`.
    pub const ENTIRE: Self = Self(f64::NEG_INFINITY, f64::INFINITY);
    /// Not an interval: `(NaN, NaN)`
    pub const NAI: Self = Self(f64::NAN, f64::NAN);

    /// Interval `[0, ∞)`.
    pub const POSITIVE: Self = Self(0., f64::INFINITY);
    /// Interval `(-∞, 0]`.
    pub const NEGATIVE: Self = Self(f64::NEG_INFINITY, 0.);

    /// Additive identity interval.
    pub const ZERO: Self = Self(0., 0.);
    /// Multiplicative identity interval.
    pub const ONE: Self = Self(1., 1.);

    /// Returns the empty interval (`∅`).
    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Returns the entire interval (`(-∞, ∞)`)
    #[must_use]
    pub const fn entire() -> Self {
        Self::ENTIRE
    }

    /// Returns the NAI interval (not an interval)
    #[must_use]
    pub const fn nai() -> Self {
        Self::NAI
    }

    /// Returns `true` if `self` is `∅` (empty)
    #[must_use]
    pub const fn is_empty(self) -> bool {
        #![expect(clippy::float_cmp, reason = "handling equal infinite endpoints")]
        self.is_nai() || self.0 > self.1 || self.0 == self.1 && self.0.is_infinite()
    }

    /// Returns `true` if `self` is `NAI` (not an interval)
    #[must_use]
    pub const fn is_nai(self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }

    /// Returns `true` if `self` is (-∞, ∞) (entire set of reals)
    #[must_use]
    pub const fn is_entire(self) -> bool {
        self.0 == f64::NEG_INFINITY && self.1 == f64::INFINITY
    }

    /// Returns `true` if `self` is a finite interval, that is, it is empty or has finite bounds.
    ///
    /// `Interval::NAI.is_finite()` returns `false`.
    ///
    /// ```
    /// use ivar::Interval;
    /// assert!(Interval::new(-2., 2.).is_finite());
    /// assert!(!Interval::new(0., f64::INFINITY).is_finite());
    /// assert!(!Interval::new(f64::NEG_INFINITY, 0.).is_finite());
    /// assert!(!Interval::ENTIRE.is_finite());
    /// assert!(Interval::EMPTY.is_finite());
    /// assert!(!Interval::NAI.is_finite());
    /// ```
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.0 > f64::NEG_INFINITY && self.1 < f64::INFINITY
    }
    /// Returns `true` if `self` is an infinite interval, that is, it has an infinite bound.
    ///
    /// `Interval::NAI.is_infinite()` returns `false`.
    ///
    /// ```
    /// use ivar::Interval;
    /// assert!(!Interval::new(-2., 2.).is_infinite());
    /// assert!(Interval::new(0., f64::INFINITY).is_infinite());
    /// assert!(Interval::new(f64::NEG_INFINITY, 0.).is_infinite());
    /// assert!(Interval::ENTIRE.is_infinite());
    /// assert!(!Interval::EMPTY.is_infinite());
    /// assert!(!Interval::NAI.is_infinite());
    /// ```
    #[must_use]
    pub const fn is_infinite(self) -> bool {
        self.0 == f64::NEG_INFINITY || self.1 == f64::INFINITY
    }
}

/// <span id="Interval-mathematical-constants">Tightest intervals enclosing common mathematical constatnts</span>
#[expect(clippy::approx_constant, reason = "definition of interval constants")]
impl Interval {
    /// The tightest interval enclosing Euler's number (`e`), the base of natural logarithms.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::E.inf().next_up(), Interval::E.sup());
    /// assert!(Interval::E.contains(core::f64::consts::E));
    ///
    /// assert_eq!(Interval::E, Interval::new_singleton(1.).exp());
    /// ```
    pub const E: Self = iv!(2.718_281_828_459_045, 2.718_281_828_459_045_5);

    /// The tightest interval enclosing `1 / π`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_1_PI.inf().next_up(), Interval::FRAC_1_PI.sup());
    /// assert!(Interval::FRAC_1_PI.contains(core::f64::consts::FRAC_1_PI));
    /// ```
    pub const FRAC_1_PI: Self = iv!(0.318_309_886_183_790_64, 0.318_309_886_183_790_7);

    /// The tightest interval enclosing `1 / sqrt(2)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_1_SQRT_2.inf().next_up(), Interval::FRAC_1_SQRT_2.sup());
    /// assert!(Interval::FRAC_1_SQRT_2.contains(core::f64::consts::FRAC_1_SQRT_2));
    /// ```
    pub const FRAC_1_SQRT_2: Self = iv!(0.707_106_781_186_547_5, 0.707_106_781_186_547_6);

    /// The tightest interval enclosing `2 / π`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_2_PI.inf().next_up(), Interval::FRAC_2_PI.sup());
    /// assert!(Interval::FRAC_2_PI.contains(core::f64::consts::FRAC_2_PI));
    /// ```
    pub const FRAC_2_PI: Self = iv!(0.636_619_772_367_581_3, 0.636_619_772_367_581_4);

    /// The tightest interval enclosing `2 / sqrt(π)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_2_SQRT_PI.inf().next_up(), Interval::FRAC_2_SQRT_PI.sup());
    /// assert!(Interval::FRAC_2_SQRT_PI.contains(core::f64::consts::FRAC_2_SQRT_PI));
    /// ```
    pub const FRAC_2_SQRT_PI: Self = iv!(1.128_379_167_095_512_6, 1.128_379_167_095_512_8);

    /// The tightest interval enclosing `π / 2`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_PI_2.inf().next_up(), Interval::FRAC_PI_2.sup());
    /// assert!(Interval::FRAC_PI_2.contains(core::f64::consts::FRAC_PI_2));
    ///
    /// ```
    pub const FRAC_PI_2: Self = iv!(1.570_796_326_794_896_6, 1.570_796_326_794_896_8);

    /// The tightest interval enclosing `π / 3`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_PI_3.inf().next_up(), Interval::FRAC_PI_3.sup());
    /// assert!(Interval::FRAC_PI_3.contains(core::f64::consts::FRAC_PI_3));
    ///
    /// assert_eq!(Interval::FRAC_PI_3, Interval::new_singleton(0.5).acos());
    /// ```
    pub const FRAC_PI_3: Self = iv!(1.047_197_551_196_597_6, 1.047_197_551_196_597_9);

    /// The tightest interval enclosing `π / 4`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_PI_4.inf().next_up(), Interval::FRAC_PI_4.sup());
    /// assert!(Interval::FRAC_PI_4.contains(core::f64::consts::FRAC_PI_4));
    ///
    /// assert_eq!(Interval::FRAC_PI_4, Interval::new_singleton(1.).atan());
    /// ```
    pub const FRAC_PI_4: Self = iv!(0.785_398_163_397_448_3, 0.785_398_163_397_448_4);

    /// The tightest interval enclosing `π / 6`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_PI_6.inf().next_up(), Interval::FRAC_PI_6.sup());
    /// assert!(Interval::FRAC_PI_6.contains(core::f64::consts::FRAC_PI_6));
    ///
    /// assert_eq!(Interval::FRAC_PI_6, Interval::new_singleton(0.5).asin());
    /// ```
    pub const FRAC_PI_6: Self = iv!(0.523_598_775_598_298_8, 0.523_598_775_598_298_9);

    /// The tightest interval enclosing `π / 8`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::FRAC_PI_8.inf().next_up(), Interval::FRAC_PI_8.sup());
    /// assert!(Interval::FRAC_PI_8.contains(core::f64::consts::FRAC_PI_8));
    /// assert_eq!(Interval::FRAC_PI_8, Interval::PI / 8.);
    /// ```
    pub const FRAC_PI_8: Self = iv!(0.392_699_081_698_724_14, 0.392_699_081_698_724_2);

    /// The tightest interval enclosing `ln 10`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::LN_10.inf().next_up(), Interval::LN_10.sup());
    /// assert!(Interval::LN_10.contains(core::f64::consts::LN_10));
    ///
    /// assert_eq!(Interval::LN_10, Interval::new_singleton(10.).ln());
    /// ```
    pub const LN_10: Self = iv!(2.302_585_092_994_045_5, 2.302_585_092_994_046);

    /// The tightest interval enclosing `ln 2`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::LN_2.inf().next_up(), Interval::LN_2.sup());
    /// assert!(Interval::LN_2.contains(core::f64::consts::LN_2));
    ///
    /// assert_eq!(Interval::LN_2, Interval::new_singleton(2.).ln());
    /// ```
    pub const LN_2: Self = iv!(0.693_147_180_559_945_3, 0.693_147_180_559_945_4);

    /// The tightest interval enclosing `log₁₀(2)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::LOG10_2.inf().next_up(), Interval::LOG10_2.sup());
    /// assert!(Interval::LOG10_2.contains(core::f64::consts::LOG10_2));
    ///
    /// assert_eq!(Interval::LOG10_2, Interval::new_singleton(2.).log10());
    /// ```
    pub const LOG10_2: Self = iv!(0.301_029_995_663_981_14, 0.301_029_995_663_981_2);

    /// The tightest interval enclosing `log₁₀(e)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::LOG10_E.inf().next_up(), Interval::LOG10_E.sup());
    /// assert!(Interval::LOG10_E.contains(core::f64::consts::LOG10_E));
    /// ```
    pub const LOG10_E: Self = iv!(0.434_294_481_903_251_8, 0.434_294_481_903_251_87);

    /// The tightest interval enclosing `log₂(10)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::LOG2_10.inf().next_up(), Interval::LOG2_10.sup());
    /// assert!(Interval::LOG2_10.contains(core::f64::consts::LOG2_10));
    ///
    /// assert_eq!(Interval::LOG2_10, Interval::new_singleton(10.).log2());
    /// ```
    pub const LOG2_10: Self = iv!(3.321_928_094_887_362, 3.321_928_094_887_362_6);

    /// The tightest interval enclosing `log₂(e)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::LOG2_E.inf().next_up(), Interval::LOG2_E.sup());
    /// assert!(Interval::LOG2_E.contains(core::f64::consts::LOG2_E));
    /// ```
    pub const LOG2_E: Self = iv!(1.442_695_040_888_963_4, 1.442_695_040_888_963_6);

    /// The tightest interval enclosing `π`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::PI.inf().next_up(), Interval::PI.sup());
    /// assert!(Interval::PI.contains(core::f64::consts::PI));
    ///
    /// assert_eq!(Interval::FRAC_PI_2 * 2., Interval::PI);
    ///
    /// // here some precision is lost because 3 = 0b11 leads to rounding
    /// assert!((Interval::FRAC_PI_3 * 3.).supset(Interval::PI));
    /// assert_eq!(Interval::FRAC_PI_4 * 4., Interval::PI);
    /// ```
    pub const PI: Self = iv!(3.141_592_653_589_793, 3.141_592_653_589_793_6);

    /// The tightest interval enclosing `sqrt(2)`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::SQRT_2.inf().next_up(), Interval::SQRT_2.sup());
    /// assert!(Interval::SQRT_2.contains(core::f64::consts::SQRT_2));
    /// assert_eq!(Interval::new_singleton(2.).sqrt(), Interval::SQRT_2);
    /// ```
    pub const SQRT_2: Self = iv!(1.414_213_562_373_095, 1.414_213_562_373_095_1);

    /// The tightest interval enclosing `2 π`.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::TAU.inf().next_up(), Interval::TAU.sup());
    /// assert!(Interval::TAU.contains(core::f64::consts::TAU));
    /// assert_eq!(Interval::PI * 2., Interval::TAU);
    /// ```
    pub const TAU: Self = iv!(6.283_185_307_179_586, 6.283_185_307_179_587);

    /// Tightest interval inclosing the rational number `N / D`.
    ///
    /// # Examples
    /// ```
    /// use ivar::{Interval, iv};
    ///
    /// // When denominator is a power of two, the number is representable exactly
    /// assert_eq!(Interval::rational(15, 1), iv!(15.));
    /// assert_eq!(Interval::rational(3, 2), iv!(1.5));
    /// assert_eq!(Interval::rational(-3, 8), iv!(-0.375));
    ///
    /// // Otherwise, exact rational involves rounding
    /// assert_eq!(Interval::rational(2, 3), iv!(2./ 3.).next_n(0, 1));
    /// assert_eq!(Interval::rational(2, 10), iv!(0.2).next_n(-1, 0));
    /// assert_eq!(Interval::rational(3, 10), iv!(0.3).next_n(0, 1));
    ///
    /// // this fraction has the same tightest enclosing interval as π constant
    /// assert_eq!(Interval::rational(245850922, 78256779), Interval::PI);
    ///
    /// // multiplying numerator or dennominator by integer
    /// // preserves number of correct decimal (or rather binary) places
    /// assert_eq!(Interval::rational({245850922 * 2}, 78256779), Interval::TAU);
    /// assert_eq!(Interval::rational(245850922, 78256779 * 3), Interval::FRAC_PI_3);
    ///
    /// // Interval::PI / 7. is rounded outwards
    /// assert_eq!(Interval::rational(245850922, 78256779 * 7), (Interval::PI / 7.).next_n(0, -1));
    /// ```
    #[must_use]
    pub const fn rational(n: i32, d: u32) -> Self {
        #![expect(
            clippy::as_conversions,
            reason = "conversion from i32 and u32 to f64 is lossless"
        )]
        // conversion from i32 and u32 to f64 is lossless
        Self(n as f64, n as f64).div(Self(d as f64, d as f64))
    }

    /// Tightest interval inclosing the rational multiple of `π`: `(N / D) * π`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::rational_times_pi(1, 1), Interval::PI);
    /// assert_eq!(Interval::rational_times_pi(2, 1), Interval::TAU);
    /// assert_eq!(Interval::rational_times_pi(1, 2), Interval::FRAC_PI_2);
    /// assert_eq!(Interval::rational_times_pi(1, 3), Interval::FRAC_PI_3);
    /// assert_eq!(Interval::rational_times_pi(1, 4), Interval::FRAC_PI_4);
    /// assert_eq!(Interval::rational_times_pi(1, 6), Interval::FRAC_PI_6);
    /// ```
    ///
    /// ```compile_fail
    /// use ivar::Interval;
    /// Interval::RATIONAL_TIMES_PI::<100, 1>; // overflow
    /// ```
    #[must_use]
    pub const fn rational_times_pi(numerator: i32, denominator: u32) -> Self {
        #![expect(
            clippy::as_conversions,
            reason = "conversion from i32 and u32 to f64 is lossless"
        )]
        let n = 245_850_922 * numerator;
        let d = 78_256_779 * denominator;
        Self(n as f64, n as f64).div(Self(d as f64, d as f64))
    }

    /// Tightest interval inclosing the rational divided by `π`: `(N / D) * 1/π`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::rational_over_pi(1, 1), Interval::FRAC_1_PI);
    /// assert_eq!(Interval::rational_over_pi(2, 1), Interval::FRAC_2_PI);
    /// ```
    #[must_use]
    pub const fn rational_over_pi(numerator: i32, denominator: u32) -> Self {
        #![expect(
            clippy::as_conversions,
            reason = "conversion from i32 and u32 to f64 is lossless"
        )]
        let n = 78_256_779 * numerator;
        let d = 245_850_922 * denominator;
        Self(n as f64, n as f64).div(Self(d as f64, d as f64))
    }
}
