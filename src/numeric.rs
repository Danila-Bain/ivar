use crate::{
    Interval,
    rounded_arithmetic::{sub_rd, sub_ru},
};

/// Implementation of numeric functions on bare intervals, defined by IEE 1788-2015 in sections 10.5.9 and 12.12.8.
impl Interval {
    /// Return the greatest lower bound of the interval `self`.
    ///
    /// Returns the lower interval bound of `self` if `self` is non-empty.
    /// Returns `+∞` if `self` is empty.
    /// Returns [`f64::NAN`] if `self` is [`Interval::NAI`].
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Accuracy*: this function always returns exact value.
    ///
    /// See also: [`Interval::sup`], [`Interval::inf_sup`].
    #[must_use]
    pub const fn inf(self) -> f64 {
        if self.is_nai() {
            f64::NAN
        } else if self.is_empty() {
            f64::INFINITY
        } else {
            self.0
        }
    }
    /// Return the smallest upper bound of the interval `self`.
    ///
    /// Returns the upper interval bound of `self` if `self` is non-empty.
    /// Returns `-∞` if `self` is empty.
    /// Returns [`f64::NAN`] if `self` is [`crate::Interval::NAI`].
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Accuracy*: this function always returns exact value.
    ///
    /// See also: [`Interval::inf`], [`Interval::inf_sup`].
    #[must_use]
    pub const fn sup(self) -> f64 {
        if self.is_nai() {
            f64::NAN
        } else if self.is_empty() {
            f64::NEG_INFINITY
        } else {
            self.1
        }
    }
    /// Simultaneously return infimum and supremum of the interval `self`.
    ///
    /// Consistent with [`Interval::inf`] and [`Interval::sup`].
    ///
    /// *Accuracy*: this function always returns exact value.
    ///
    /// The function is *not* a part of IEEE 1788-2015.
    #[must_use]
    pub const fn inf_sup(self) -> (f64, f64) {
        if self.is_nai() {
            (f64::NAN, f64::NAN)
        } else if self.is_empty() {
            (f64::INFINITY, f64::NEG_INFINITY)
        } else {
            (self.0, self.1)
        }
    }
    /// Returns the midpoint of the interval `self`.
    ///
    /// | `self`                | output     |
    /// --------------------------------------
    /// | empty or nai          | NaN        |
    /// | entire `(-∞,+∞)`      | 0.         |
    /// | `(-∞, x]`, `x` finite | `f64::MIN` |
    /// | `[x, +∞)`, `x` finite | `f64::MAX` |
    /// | `[x, y]`, `x` and `y` finite | `f64::midpoint(x, y)` : `(x + y)/2` rounded to nearest |
    /// --------------------------------------
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Note*: returned value is guaranteed to be finite and contained in `self` for any non-empty `self`.
    ///
    /// *Accuracy*: The result is rounded to nearest.
    ///
    /// # Examples
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::new(-1., 1.).mid(), 0.);
    /// assert_eq!(Interval::ENTIRE.mid(), 0.);
    /// assert_eq!(Interval::new(f64::MIN/2., f64::MAX).mid(), f64::MAX/4.);
    /// assert!(Interval::NAI.mid().is_nan());
    /// assert!(Interval::EMPTY.mid().is_nan());
    /// ```
    #[must_use]
    pub const fn mid(self) -> f64 {
        if self.is_empty() {
            f64::NAN
        } else if self.is_entire() {
            0.
        } else if self.0 == f64::NEG_INFINITY {
            f64::MIN
        } else if self.1 == f64::INFINITY {
            f64::MAX
        } else {
            f64::midpoint(self.0, self.1)
        }
    }
    /// Returns width `sup(self) - inf(self)` of the interval `self` rounded towards `+∞`.
    ///
    /// Returns `NaN` if `self` is empty.
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Accuracy*: The result is rounded towards `+∞`.
    ///
    /// See also: [`Interval::rad`].
    ///
    /// # Examples
    /// ```
    /// use ivar::Interval;
    ///
    /// assert_eq!(Interval::new(-1., 1.).wid(), 2.);
    /// assert_eq!(Interval::ENTIRE.wid(), f64::INFINITY);
    /// assert_eq!(Interval::new(f64::MIN/2., f64::MAX/2.).wid(), f64::MAX);
    /// assert!(Interval::EMPTY.wid().is_nan());
    /// assert!(Interval::NAI.wid().is_nan());
    /// ```
    #[must_use]
    pub const fn wid(self) -> f64 {
        if self.is_empty() {
            f64::NAN
        } else {
            sub_ru(self.1, self.0)
        }
    }

    /// Returns the radius of the interval `self`: the smallest value `r` such that `self` is contained in the exact interval `[m - r, m + r]`, where `m` is the value returned by [`Interval::mid`].
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Accuracy*: The result is rounded towards `+∞`.
    ///
    /// *Note*: `self.rad()` is finite for all bounded and non-empty `self`.
    ///
    /// # Examples
    /// ```
    /// use ivar::iv;
    ///
    /// assert_eq!(iv!(-100., 100.).rad(), 100.);
    /// assert_eq!(iv!(f64::MIN, f64::MAX).rad(), f64::MAX);
    ///
    /// assert!(iv!().rad().is_nan());
    /// assert!(iv!(nai).rad().is_nan());
    /// ```
    #[must_use]
    pub const fn rad(self) -> f64 {
        if self.is_empty() {
            f64::NAN
        } else {
            let mid = self.mid(); // guaranteed to be finite
            f64::max(sub_ru(self.1, mid), sub_ru(mid, self.0)) // is bounded if self is bounded
        }
    }
    /// Return the midpoint and the width of the interval `self`.
    ///
    /// Consistent with [`Interval::mid`] and [`Interval::wid`].
    ///
    /// The function is *not* a part of IEEE 1788-2015.
    #[must_use]
    pub const fn mid_wid(self) -> (f64, f64) {
        if self.is_empty() {
            (f64::NAN, f64::NAN)
        } else {
            let mid = if self.is_entire() {
                0.
            } else if self.0 == f64::NEG_INFINITY {
                f64::MIN
            } else if self.1 == f64::INFINITY {
                f64::MAX
            } else {
                f64::midpoint(self.0, self.1)
            };

            (mid, sub_ru(self.1, self.0))
        }
    }
    /// Return the midpoint and the radius of the interval `self`.
    ///
    /// Consistent with [`Interval::mid`] and [`Interval::rad`].
    ///
    /// The function is *required* by IEEE 1788-2015 (Section 10.5.9).
    #[must_use]
    pub const fn mid_rad(self) -> (f64, f64) {
        if self.is_empty() {
            (f64::NAN, f64::NAN)
        } else {
            let mid = if self.is_entire() {
                0.
            } else if self.0 == f64::NEG_INFINITY {
                f64::MIN
            } else if self.1 == f64::INFINITY {
                f64::MAX
            } else {
                f64::midpoint(self.0, self.1)
            };

            let rad = f64::max(sub_ru(self.1, mid), sub_ru(mid, self.0));
            (mid, rad)
        }
    }
    /// Return the magnitude of the interval `self`, that is, `sup {|x| : x ∈ self}` for non-empty `self`.
    ///
    /// Returns `f64::NAN` if `self` is empty.
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Accuracy*: exact.
    #[must_use]
    pub const fn mag(self) -> f64 {
        if self.is_empty() {
            f64::NAN
        } else {
            f64::max(self.0.abs(), self.1.abs())
        }
    }
    /// Return the mignitude of the interval `self`, that is, `inf {|x| : x ∈ self}` for non-empty `self`.
    ///
    /// Returns `f64::NAN` if `self` is empty.
    ///
    /// Returns `+0` with positive sign if `self` contains zero.
    ///
    /// The function is *required* by IEEE 1788-2015 (Sections 10.5.9 and 12.12.8).
    ///
    /// *Accuracy*: exact.
    #[must_use]
    pub const fn mig(self) -> f64 {
        if self.is_empty() {
            f64::NAN
        } else if self.0 > 0. {
            self.0
        } else if self.1 < 0. {
            #[expect(clippy::float_arithmetic, reason = "negation is exact")]
            -self.1
        } else {
            0.
        }
    }

    /// Returns the Hausdorph distance between `self` and `other` rounding to nearest.
    ///
    /// Returned value is given by the formula
    /// `max( sup{d(x, other) : x ∈ self}, sup{d(y, self) : y ∈ other})`,
    /// where `d(x, Y) = inf{d(x, y) : y ∈ Y}`.
    #[must_use]
    pub const fn hausdorph_distance(self, other: Self) -> f64 {
        #[expect(clippy::float_arithmetic, reason = "rounding to nearest is needed")]
        f64::max(
            f64::max(
                f64::min((self.0 - other.0).abs(), (self.0 - other.1).abs()),
                f64::min((self.1 - other.0).abs(), (self.1 - other.1).abs()),
            ),
            f64::max(
                f64::min((self.0 - other.0).abs(), (self.1 - other.0).abs()),
                f64::min((self.0 - other.1).abs(), (self.1 - other.1).abs()),
            ),
        )
    }

    /// Returns the Hausdorph distance between `self` and `other` rounded up.
    #[must_use]
    pub const fn hausdorph_distance_ru(self, other: Self) -> f64 {
        const fn distance_ru(x: f64, y: f64) -> f64 {
            if x > y { sub_ru(x, y) } else { sub_ru(y, x) }
        }
        f64::max(
            f64::max(
                f64::min(distance_ru(self.0, other.0), distance_ru(self.0, other.1)),
                f64::min(distance_ru(self.1, other.0), distance_ru(self.1, other.1)),
            ),
            f64::max(
                f64::min(distance_ru(self.0, other.0), distance_ru(self.1, other.0)),
                f64::min(distance_ru(self.0, other.1), distance_ru(self.1, other.1)),
            ),
        )
    }
    /// Returns the Hausdorph distance between `self` and `other` rounded down.
    #[must_use]
    pub const fn hausdorph_distance_rd(self, other: Self) -> f64 {
        const fn distance_rd(x: f64, y: f64) -> f64 {
            if x > y { sub_rd(x, y) } else { sub_rd(y, x) }
        }
        f64::max(
            f64::max(
                f64::min(distance_rd(self.0, other.0), distance_rd(self.0, other.1)),
                f64::min(distance_rd(self.1, other.0), distance_rd(self.1, other.1)),
            ),
            f64::max(
                f64::min(distance_rd(self.0, other.0), distance_rd(self.1, other.0)),
                f64::min(distance_rd(self.0, other.1), distance_rd(self.1, other.1)),
            ),
        )
    }
}
