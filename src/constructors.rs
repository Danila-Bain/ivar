use crate::{
    Interval,
    rounded_arithmetic::{add_ru, sub_rd},
};

/// Construction methods for [`Interval`]
impl Interval {
    /// Create a new interval from its bounds.
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert!(Interval::new(0., 1.).contains(0.5));
    /// assert!(Interval::new(-2., 1.).contains(-2.));
    /// assert!(Interval::new(-2., 3.).contains(3.));
    /// assert!(Interval::new(f64::NEG_INFINITY, 0.).contains(-1e300));
    /// assert!(Interval::new(f64::INFINITY, 0.).is_empty()); // invalid but numerical bounds
    /// assert!(Interval::new(0., f64::NAN).is_nai()); // NAN bound produces NAI
    /// assert_eq!(Interval::new(f64::NEG_INFINITY, f64::INFINITY), Interval::ENTIRE);
    ///
    /// assert!(Interval::new(f64::INFINITY, f64::INFINITY).is_empty());
    /// assert!(Interval::new(f64::NEG_INFINITY, f64::NEG_INFINITY).is_empty());
    /// ```
    ///
    /// Note that in last two examples intervals are empty and not singletones {$+\infty$} and {$-\infty$} because by intervals we consider closed and connected subsets of $\R$ (not $\XR$), so it is natural that the set of points $x \in \R$ such that $+\infty \leq x \leq +\infty$ is empty.
    #[must_use]
    pub const fn new(inf: f64, sup: f64) -> Self {
        #![expect(clippy::float_cmp, reason = "handling equal infinite endpoints")]
        if inf.is_nan() || sup.is_nan() {
            Self::NAI
        } else if inf > sup || (inf == sup && inf.is_infinite()) {
            Self::EMPTY
        } else {
            Self(inf, sup)
        }
    }

    /// Create a new interval connecting two points.
    ///
    /// The function is insensitive to the order of points.
    #[must_use]
    pub const fn new_unordered(a: f64, b: f64) -> Self {
        #![expect(clippy::float_cmp, reason = "handling equal infinite endpoints")]
        if a.is_nan() || b.is_nan() {
            Self::NAI
        } else if a == b && a.is_infinite() {
            Self::EMPTY
        } else if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }

    /// Returns an interval containing the single point `p`.
    /// If `p` is NaN, return [`Interval::NAI`].
    ///
    /// If `p` is infinite, return [`Interval::EMPTY`].
    ///
    /// If `p` is finite, return the interval `[p, p]`.
    ///
    /// No rounding occurs, `p` is treated as an exact value.
    /// For tighest intervals containing common mathematical constants, see [associated constants](#Interval-mathematical-constants).
    ///
    /// This function is *not* a part of IEEE 1788-2015.
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::new_singleton(0.), Interval::new(0., 0.));
    /// assert!(Interval::new_singleton(1.).is_singleton());
    /// assert!(!Interval::new_singleton(f64::NAN).is_singleton());
    /// assert!(Interval::new_singleton(f64::NAN).is_nai());
    ///
    /// assert!(!Interval::new_singleton(f64::INFINITY).is_singleton());
    /// assert!(!Interval::new_singleton(f64::INFINITY).is_nai());
    /// assert!(Interval::new_singleton(f64::INFINITY).is_empty());
    /// ```
    #[must_use]
    pub const fn new_singleton(p: f64) -> Self {
        if p.is_nan() {
            Self::NAI
        } else if p.is_infinite() {
            Self::EMPTY
        } else {
            Self(p, p)
        }
    }

    /// Returns the tightest interval enclosing `[mid - rad, mid + rad]`.
    ///
    /// If either `mid` or `rad` is NaN, or both `mid` and `rad` are infinite, returns [`Interval::NAI`].
    ///
    /// If `rad < 0`, returns [`Interval::EMPTY`].
    ///
    /// If `rad` is finite and `mid` is infinite, returns [`Interval::EMPTY`].
    ///
    /// This function is *not* a part of IEEE 1788-2015.
    #[must_use]
    pub const fn from_mid_rad(mid: f64, rad: f64) -> Self {
        // mid is always finite
        if mid.is_nan() || rad.is_nan() || mid.is_infinite() {
            Self::NAI
        } else if rad < 0. {
            Self::EMPTY
        } else {
            Self(sub_rd(mid, rad), add_ru(mid, rad))
        }
    }

    /// Returns the tightest interval enclosing the given set of points, ignoring NaN points as [`f64::min`]  and [`f64::max`] functions do.
    ///
    /// Returns [`Interval::EMPTY`] if `points` is empty or all of them are NaN.
    ///
    /// *Accuracy mode*: *tightest*. This function always returns exact values.
    ///
    /// This function is *not* a part of IEEE 1788-2015.
    ///
    /// # Examples
    ///
    /// ```
    /// use ivar::{Interval, iv};
    ///
    /// assert!(Interval::hull_of_points(&[]).is_empty());
    /// assert_eq!(Interval::hull_of_points(&[1.]), iv!(1.));
    /// assert_eq!(Interval::hull_of_points(&[1., -1., 2., -2.]), iv!(-2., 2.));
    /// assert_eq!(Interval::hull_of_points(&[1., -1., f64::NAN, 2., -2.]), iv!(-2., 2.));
    /// assert_eq!(Interval::hull_of_points(&[f64::NAN, f64::NAN, 3., f64::NAN]), iv!(3.));
    /// assert_eq!(Interval::hull_of_points(&[f64::NAN, f64::NAN, f64::NAN]), Interval::EMPTY);
    /// assert_eq!(Interval::hull_of_points(&[f64::INFINITY, f64::INFINITY]), Interval::EMPTY);
    /// ```
    #[must_use]
    pub fn hull_of_points(points: &[f64]) -> Self {
        if points.is_empty() {
            Self::EMPTY
        } else {
            let min = points.iter().copied().fold(f64::INFINITY, f64::min);
            let max = points.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            Self::new(min, max)
        }
    }
}

/// Convenience macro for interval construction.
///
/// ```
/// use ivar::{Interval, iv};
///
/// // default is empty
/// assert_eq!(iv!(), Interval::EMPTY);
///
/// // singleton
/// assert_eq!(iv!(42.), Interval::new_singleton(42.));
///
/// // named
/// assert_eq!(iv!(empty), Interval::EMPTY);
/// assert_eq!(iv!(entire), Interval::ENTIRE);
/// assert!(iv!(nai).is_nai());
///
/// // inf-sup notation
/// assert_eq!(iv![0., 1.], Interval::new(0., 1.));
/// assert_eq!(iv![1.,], Interval::new(1., f64::INFINITY));
/// assert_eq!(iv![1., inf], Interval::new(1., f64::INFINITY));
/// assert_eq!(iv![-inf, -1.], Interval::new(f64::NEG_INFINITY, -1.));
/// assert_eq!(iv![,-1.], Interval::new(f64::NEG_INFINITY, -1.));
/// assert_eq!(iv![-inf, inf], Interval::ENTIRE);
/// assert_eq!(iv![,], Interval::ENTIRE);
///
/// // range notation
/// assert_eq!(iv!(1. .. 2.), Interval::new(1., 2.));
/// assert_eq!(iv!(1. ..), Interval::new(1., f64::INFINITY));
/// assert_eq!(iv!(.. -1.), Interval::new(f64::NEG_INFINITY, -1.));
/// assert_eq!(iv!(..), Interval::ENTIRE);
///
/// // mid-rad notation
/// assert_eq!(iv!(10. +- 2.), Interval::new(8., 12.));
/// assert_eq!(iv!(+-2.), Interval::new(-2., 2.));
///
/// // for range notation (`iv!(1. .. 2.)`) and mid-rad notation (`iv!(1. +- 0.1)`),
/// // complex expressions (not literals or singe idents)
/// // in first position need wrapping in block with curly braces.
/// assert_eq!(iv!({10. - 2.} .. 10. + 2.), Interval::new(8., 12.));
/// assert_eq!(iv!({10. - 2.} +- 2.), Interval::new(6., 10.));
///
/// assert_eq!(iv!(10. - 2., 10. + 2.), Interval::new(8., 12.));
/// ```
#[macro_export]
macro_rules! iv {
    () => {
        $crate::Interval::EMPTY
    };
    //
    // named
    //
    (nai) => {
        $crate::Interval::NAI
    };
    (empty) => {
        $crate::Interval::EMPTY
    };
    (entire) => {
        $crate::Interval::ENTIRE
    };
    //
    // [inf, sup] notation
    //
    (,) => {
        $crate::Interval::ENTIRE
    };
    (-inf, inf $(,)?) => {
        $crate::Interval::ENTIRE
    };
    ($inf:expr, inf $(,)?) => {
        $crate::Interval::new($inf, f64::INFINITY)
    };
    (-inf, $sup:expr $(,)?) => {
        $crate::Interval::new(f64::NEG_INFINITY, $sup)
    };
    ($inf:expr,  $(,)?) => {
        $crate::Interval::new($inf, f64::INFINITY)
    };
    (, $sup:expr $(,)?) => {
        $crate::Interval::new(f64::NEG_INFINITY, $sup)
    };
    ($inf:expr, $sup:expr $(,)?) => {
        $crate::Interval::new($inf, $sup)
    };
    //
    // (mid +- rad) notation
    //
    ($mid:literal +- $rad:expr) => {
        $crate::Interval::from_mid_rad($mid, $rad)
    };
    ($mid:ident +- $rad:expr) => {
        $crate::Interval::from_mid_rad($mid, $rad)
    };
    ($mid:block +- $rad:expr) => {
        $crate::Interval::from_mid_rad($mid, $rad)
    };
    (+- $rad:expr) => {
        $crate::Interval::from_mid_rad(0., $rad)
    };
    //
    // (inf .. sup) notation
    //
    (..) => {
        $crate::Interval::ENTIRE
    };
    ($inf:literal ..) => {
        $crate::Interval::new($inf, f64::INFINITY)
    };
    ($inf:ident ..) => {
        $crate::Interval::new($inf, f64::INFINITY)
    };
    ($inf:block ..) => {
        $crate::Interval::new($inf, f64::INFINITY)
    };
    ($inf:literal .. $sup:expr) => {
        $crate::Interval::new($inf, $sup)
    };
    ($inf:ident .. $sup:expr) => {
        $crate::Interval::new($inf, $sup)
    };
    ($inf:block .. $sup:expr) => {
        $crate::Interval::new($inf, $sup)
    };
    (.. $sup:expr) => {
        $crate::Interval::new(f64::NEG_INFINITY, $sup)
    };
    //
    // singleton
    //
    ($point:expr) => {
        $crate::Interval::new_singleton($point)
    };
}
/// More descriptive alias for macro [`iv!`].
#[macro_export]
macro_rules! interval {
    ($($t:tt)*) => { $crate::iv!($($t)*) }
}
