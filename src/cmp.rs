#![expect(clippy::float_cmp, reason = "equal endpoints must be handled")]
#![expect(
    clippy::suspicious_operation_groupings,
    reason = "opposite endpoins of intervals are compared"
)]
use core::cmp::Ordering;

use crate::Interval;

/// The overlapping state between intervals, returned by [`Interval::overlap`].
///
/// # Quick Reference
///
/// `self` relative to `rhs`:
///
/// ```text
///                            rhs
///                         c       d
///                         •———————•
///      ┌─          a   b  :       :
///      │  Before   •———•  :       :
///      │  Meets       •———•       :                                   rhs
///      │  Overlaps      •———•     :                                   c=d
///      │  Starts          •———•   :                                    •
///      │  ^^^^^^          •       :                  ┌─         a   b  :
///      │  ContainedBy     : •———• :                  │  Before  •———•  :
///      │  Finishes        :   •———•                  │  Equal          •
/// self │  ^^^^^^^^        :       •             self │  FinishedBy •———•   
///      │  Equal           •———————•                  │  Contains     •———•  
///      │  FinishedBy    •—————————•                  │  StartedBy      •———•   
///      │  Contains      •———————————•                │  After          :  •———•  
///      │  StartedBy       •—————————•                └─                :  a   b
///      │  OverlappedBy    :     •———•                                  •
///      │  MetBy           :       •———•                               c=d
///      │  After           :       :  •———•  
///      └─                 :       :  a   b
///                         •———————•
///                         c       d
/// ```
#[rustfmt::skip]
#[expect(clippy::unusual_byte_groupings, reason = "bit groupings have different meanings and sizes")]
#[derive(Copy, Clone, Debug)]
pub enum Overlap {
    /// Intervals are equal
    /// ```text
    /// a      b
    /// ⏺------⏺
    /// :      :
    /// ⏺------⏺
    /// c      d
    /// ```
    Equal        = 0b0000_11_00_00_11,
    /// Both interval are empty
    BothEmpty    = 0b0001_11_10_01_11,
    /// The first interval is empty and the second interval is not empty.
    FirstEmpty   = 0b0001_10_10_01_01,
    /// The first interval is not empty and the second interval is empty.
    SecondEmpty  = 0b0001_01_10_01_10,
    /// The first interval is before the second interval 
    /// ```text
    /// a      b
    /// ⏺------⏺  :      :
    /// :      :  :      :
    /// :      :  ⏺------⏺
    ///           c      d
    /// ```
    Before       = 0b0001_01_01_01_01,
    /// The first interval is after the second interval 
    /// ```text
    ///           a      b
    /// :      :  ⏺------⏺ 
    /// :      :  :      :    
    /// ⏺------⏺  :      :
    /// c      d     
    /// ```
    After        = 0b0001_10_10_10_10,
    /// The first interval starts the second interval 
    /// ```text
    /// a   b
    /// ⏺---⏺ 
    /// :   : 
    /// ⏺-------⏺
    /// c       d
    /// ```
    Starts       = 0b0010_11_01_10_01,
    /// The first interval is contained by the second interval 
    /// ```text
    ///   a   b
    ///   ⏺---⏺ 
    ///   :   : 
    /// ⏺-------⏺
    /// c      d
    /// ```
    ContainedBy  = 0b0010_10_01_10_01,
    /// The first interval finishes the second interval 
    /// ```text
    ///     a   b
    ///     ⏺---⏺
    ///     :   :
    /// ⏺-------⏺
    /// c       d
    /// ```
    Finishes     = 0b0010_10_01_10_11,
    StartedBy    = 0b0100_11_01_10_10,
    Contains     = 0b0100_01_01_10_10,
    FinishedBy   = 0b0100_01_01_10_11,
    Meets        = 0b1000_01_01_11_01,
    Overlaps     = 0b1000_01_01_10_01,
    OverlappedBy = 0b1000_10_01_10_10,
    MetBy        = 0b1000_10_11_10_10,
}

impl Overlap {
    #![allow(clippy::as_conversions, reason = "getting bits of enum values")]

    #[must_use]
    pub const fn disjoint(self) -> bool {
        self as u16 >> 8 == 1
    }
    #[must_use]
    pub const fn proper_subset(self) -> bool {
        self as u16 >> 8 == 2
    }
    #[must_use]
    pub const fn proper_supset(self) -> bool {
        self as u16 >> 8 == 4
    }
    #[must_use]
    pub const fn partial_overlap(self) -> bool {
        self as u16 >> 8 == 8
    }
}

impl Interval {
    /// Determine whether the interval contains a single element
    ///
    /// ```
    /// use ivar::Interval;
    ///
    /// assert!(Interval::new(0., 0.).is_singleton());
    /// assert!(Interval::new(9., 9.).is_singleton());
    /// assert!(Interval::new(f64::MAX, f64::MAX).is_singleton());
    /// assert!(!Interval::new(f64::NAN, f64::NAN).is_singleton());
    /// assert!(!Interval::new(f64::INFINITY, f64::INFINITY).is_singleton());
    /// assert!(!Interval::new(f64::NEG_INFINITY, f64::NEG_INFINITY).is_singleton());
    /// ```
    #[must_use]
    pub const fn is_singleton(self) -> bool {
        self.0 == self.1 && self.0.is_finite()
    }
    #[must_use]
    pub const fn contains(self, value: f64) -> bool {
        !self.is_nai() && value.is_finite() && value >= self.0 && value <= self.1
    }
    #[must_use]
    pub const fn interior_contains(self, value: f64) -> bool {
        !self.is_nai() && value > self.0 && value < self.1
    }

    #[must_use]
    pub const fn overlap(self, other: Self) -> Overlap {
        #![expect(
            clippy::collapsible_else_if,
            reason = "the structure of conditions is clearer this way"
        )]
        if self.is_empty() {
            if other.is_empty() {
                Overlap::BothEmpty
            } else {
                Overlap::FirstEmpty
            }
        } else if other.is_empty() {
            Overlap::SecondEmpty
        } else if self.1 < other.1 {
            if self.0 < other.0 {
                if self.1 < other.0 {
                    Overlap::Before
                } else if self.1 == other.0 {
                    Overlap::Meets
                } else {
                    Overlap::Overlaps
                }
            } else if self.0 == other.0 {
                Overlap::Starts
            } else {
                Overlap::ContainedBy
            }
        } else if self.1 == other.1 {
            if self.0 < other.0 {
                Overlap::FinishedBy
            } else if self.0 == other.0 {
                Overlap::Equal
            } else {
                Overlap::Finishes
            }
        } else {
            if self.0 < other.0 {
                Overlap::Contains
            } else if self.0 == other.0 {
                Overlap::StartedBy
            } else {
                if self.0 < other.1 {
                    Overlap::OverlappedBy
                } else if self.0 == other.1 {
                    Overlap::MetBy
                } else {
                    Overlap::After
                }
            }
        }
    }
    #[must_use]
    pub const fn equal(self, other: Self) -> bool {
        !self.is_nai()
            && !other.is_nai()
            && (self.is_empty() && other.is_empty() || self.0 == other.0 && self.1 == other.1)
    }
    #[must_use]
    pub const fn le_weak(self, other: Self) -> bool {
        !self.is_empty() && !other.is_empty() && self.0 <= other.0 && self.1 <= other.1
    }
    #[must_use]
    pub const fn ge_weak(self, other: Self) -> bool {
        other.le_weak(self)
    }
    #[must_use]
    pub const fn le_all(self, other: Self) -> bool {
        !self.is_nai() && !other.is_nai() && self.1 <= other.0
    }
    #[must_use]
    pub const fn ge_all(self, other: Self) -> bool {
        other.le_all(self)
    }
    #[must_use]
    pub const fn lt_weak(self, other: Self) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && (self.0 == f64::NEG_INFINITY || self.0 < other.0)
            && (self.1 < other.1 || other.1 == f64::INFINITY)
    }
    #[must_use]
    pub const fn gt_weak(self, other: Self) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && (self.0 > other.0 || other.0 == f64::NEG_INFINITY)
            && (self.1 == f64::INFINITY || self.1 > other.1)
    }
    #[must_use]
    pub const fn lt_all(self, other: Self) -> bool {
        !self.is_nai() && !other.is_nai() && self.1 < other.0
    }
    #[must_use]
    pub const fn gt_all(self, other: Self) -> bool {
        other.lt_all(self)
    }
    #[must_use]
    pub const fn subset(self, other: Self) -> bool {
        !self.is_nai() && !other.is_empty() && self.0 >= other.0 && self.1 <= other.1
    }
    #[must_use]
    pub const fn interior(self, other: Self) -> bool {
        !self.is_nai()
            && !other.is_empty()
            && (self.0 > other.0 || other.0 == f64::NEG_INFINITY)
            && (self.1 < other.1 || other.1 == f64::INFINITY)
    }
    #[must_use]
    pub const fn proper_subset(self, other: Self) -> bool {
        !self.is_nai() && !other.is_empty() && self.0 > other.0 && self.1 < other.1
    }
    #[must_use]
    pub const fn supset(self, other: Self) -> bool {
        other.subset(self)
    }
    #[must_use]
    pub const fn exterior(self, other: Self) -> bool {
        other.interior(self)
    }
    #[must_use]
    pub const fn proper_supset(self, other: Self) -> bool {
        other.proper_subset(self)
    }
    #[must_use]
    pub const fn disjoint(self, other: Self) -> bool {
        !self.is_nai()
            && !other.is_nai()
            && (self.is_empty() || other.is_empty() || self.1 < other.0 || self.0 > other.1)
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other)
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lt_all(*other) {
            Some(Ordering::Less)
        } else if self.gt_all(*other) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
