use crate::Interval;

/// Implementation of set operations on [`Interval`]
impl Interval {
    /// Return the intersection `self ∩ other`.
    ///
    /// Returns [`crate::Interval::NAI`] if either of inputs is [`crate::Interval::NAI`].
    ///
    /// This function is *required* by IEEE 1788-2015 (Sections 10.5.7 and 12.12.6), where it is called `intersection`.
    ///
    /// *Accuracy*: exact.
    ///
    /// See also: [`crate::Interval::hull`].
    ///
    /// # Examples
    /// ```
    /// use ivar::iv;
    ///
    /// assert_eq!(iv!(-1., 3.).inter(iv!(empty)), iv!(empty));
    /// assert_eq!(iv!(-1., 3.).inter(iv!(entire)), iv!(-1., 3.));
    /// assert_eq!(iv!(-1., 3.).inter(iv!(-3., 1.)), iv!(-1., 1.));
    /// assert_eq!(iv!(-1., 0.).inter(iv!(0., 1.)), iv!(0.));
    /// assert_eq!(iv!(-2., -1.).inter(iv!(1., 2.)), iv!(empty));
    /// assert_eq!(iv!(-1. ..).inter(iv!(.. 1.)), iv!(-1., 1.));
    /// ```
    #[must_use]
    pub const fn inter(self, other: Self) -> Self {
        #![expect(
            clippy::suspicious_operation_groupings,
            reason = "comparing opposite interval bounds"
        )]
        if self.is_nai() || other.is_nai() {
            Self::NAI
        } else if self.is_empty() || other.is_empty() || self.1 < other.0 || self.0 > other.1 {
            Self::EMPTY
        } else {
            Self(f64::max(self.0, other.0), f64::min(self.1, other.1))
        }
    }
    /// Return the tightest interval containing the union `self ∪ other`.
    ///
    /// Returns [`crate::Interval::NAI`] if either of inputs is [`crate::Interval::NAI`].
    ///
    /// This function is *required* by IEEE 1788-2015 (Sections 10.5.7 and 12.12.6), where it is called `convexHull`.
    ///
    /// *Accuracy*: exact.
    ///
    /// See also: [`crate::Interval::inter`].
    ///
    /// # Examples
    /// ```
    /// use ivar::iv;
    ///
    /// assert_eq!(iv!(-1., 3.).hull(iv!(empty)), iv!(-1., 3.));
    /// assert_eq!(iv!(-1., 3.).hull(iv!(entire)), iv!(entire));
    /// assert_eq!(iv!(-1., 3.).hull(iv!(-3., 1.)), iv!(-3., 3.));
    /// assert_eq!(iv!(-1., 0.).hull(iv!(0., 1.)), iv!(-1., 1.));
    /// assert_eq!(iv!(-2.,-1.).hull(iv!(1., 2.)), iv!(-2., 2.));
    /// assert_eq!(iv!(-1. ..).hull(iv!(.. 1.)), iv!(entire));
    /// ```
    #[must_use]
    pub const fn hull(self, other: Self) -> Self {
        if self.is_nai() || other.is_nai() {
            Self::NAI
        } else if self.is_empty() {
            other
        } else if other.is_empty() {
            self
        } else {
            Self(f64::min(self.0, other.0), f64::max(self.1, other.1))
        }
    }

    /// Return the tightest interval, the interior of which contains `self`.
    ///
    /// Such interval is constructed by replacing `self = [a, b]` with `[a.next_down(), b.next_up()]` by using functions [`f64::next_down`] and [`f64::next_up`], which replaces finite bound by next representable numbers in outward direction, while keeping infinite bounds unchanged.
    ///
    /// Returns `self` if `self` is empty (or nai).
    ///
    /// This function is *not* a part of IEEE 1788-2015, although it is *defined* there in section 12.10.1 to define accuracy modes.
    ///
    /// The intended usage of this function is to convert rounded-to-nearest values to intervals that are guaranteed to contain the exact value (although with one extra ULP in width):
    /// ```
    /// use ivar::Interval;
    ///
    /// let silver_ratio = Interval::new_singleton(2.41421_35623_73095_04880).next_out();
    /// assert!(silver_ratio.supset(1. + Interval::SQRT_2));
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use ivar::iv;
    ///
    /// assert_eq!(iv!(1.).next_out(), iv!(1. - f64::EPSILON/2., 1. + f64::EPSILON));
    /// assert_eq!(iv!(f64::MIN, f64::MAX).next_out(), iv!(f64::NEG_INFINITY, f64::INFINITY));
    ///
    /// ```
    #[must_use]
    pub const fn next_out(self) -> Self {
        if self.is_empty() {
            self
        } else {
            Self(self.0.next_down(), self.1.next_up())
        }
    }

    #[must_use]
    pub fn next_n(self, n_inf: i32, n_sup: i32) -> Self {
        if self.is_empty() {
            self
        } else {
            let Self(mut inf, mut sup) = self;
            for _ in 0..-n_inf {
                inf = inf.next_down();
            }
            for _ in 0..n_inf {
                inf = inf.next_up();
            }
            for _ in 0..n_sup {
                sup = sup.next_up();
            }
            for _ in 0..-n_sup {
                sup = sup.next_down();
            }
            Self::new(inf, sup) // emptiness checks are needed
        }
    }

    /// Return tightest interval containing the set difference `self ∖ other`.
    #[must_use]
    pub const fn set_difference_hull(self, other: Self) -> Self {
        if other.is_nai() {
            other
        } else if other.0 <= self.0 {
            if other.1 < self.1 {
                Self(f64::max(self.0, other.1), self.1)
            } else {
                Self::EMPTY
            }
        } else if other.1 >= self.1 {
            // other.0 > self.0
            Self(self.0, f64::min(self.1, other.0))
        } else {
            // self.0 < other.0 && other.1 < self.1
            // so `other ⊂ self` or `other` is empty
            self
        }
    }
    /// Returns the connected components of the set difference `self ∖ other`.
    ///
    /// If `self ∖ other` is connected, returns `(self ∖ other, ∅)`.
    /// If `self ∖ other` has two connected components `I < J`, returns `(I, J)`
    #[must_use]
    pub const fn set_difference_pair(self, other: Self) -> (Self, Self) {
        if other.is_nai() {
            (Self::NAI, Self::NAI)
        } else if other.0 <= self.0 {
            if other.1 < self.1 {
                (Self(f64::max(self.0, other.1), self.1), Self::EMPTY)
            } else {
                (Self::EMPTY, Self::EMPTY)
            }
        } else if other.1 >= self.1 {
            // other.0 > self.0
            (Self(self.0, f64::min(self.1, other.0)), Self::EMPTY)
        } else if other.is_empty() {
            (self, Self::EMPTY)
        } else {
            // self.0 < other.0 <= other.1 < self.1
            (Self(self.0, other.0), Self(other.1, self.1))
        }
    }

    /// Returns the connected components of the compliment `ℝ ∖ self`.
    #[must_use]
    pub const fn compliment_pair(self) -> (Self, Self) {
        if self.is_nai() {
            (Self::NAI, Self::NAI)
        } else if self.is_empty() {
            (Self::ENTIRE, Self::EMPTY)
        } else if self.is_entire() {
            (Self::EMPTY, Self::EMPTY)
        } else if self.0 == f64::NEG_INFINITY {
            (Self(self.1, f64::INFINITY), Self::EMPTY)
        } else if self.1 == f64::INFINITY {
            (Self(f64::NEG_INFINITY, self.0), Self::EMPTY)
        } else {
            (Self(f64::NEG_INFINITY, self.0), Self(self.1, f64::INFINITY))
        }
    }

    /// Returns the connected components of symmetric difference `self Δ other`.
    #[must_use]
    pub const fn symmetric_difference_pair(self, other: Self) -> (Self, Self) {
        #![expect(clippy::float_cmp, reason = "coinciding endpoints must be handled")]
        if self.1 < other.0 {
            (self, other)
        } else if self.0 > other.1 {
            (other, self)
        } else if self.0 == other.0 {
            if self.1 < other.1 {
                (Self(self.1, other.1), Self::EMPTY)
            } else {
                (Self(other.1, self.1), Self::EMPTY)
            }
        } else if self.1 == other.1 {
            if self.0 < other.0 {
                (Self(self.0, other.0), Self::EMPTY)
            } else {
                (Self(other.0, self.0), Self::EMPTY)
            }
        } else {
            let left = if self.0 < other.0 {
                Self(self.0, other.0)
            } else {
                Self(other.0, self.0)
            };

            let right = if self.1 < other.1 {
                Self(self.1, other.1)
            } else {
                Self(other.1, self.1)
            };

            (left, right)
        }
    }
}
