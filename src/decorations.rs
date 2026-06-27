use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::{
    Interval, Overlap,
    rounded_arithmetic::{add_ru, sub_rd},
};

/// Information about domain and continuity of function that returned the attached interval.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Decoration {
    /// Ill-formed: not an interval (NAI)
    Ill = 0b_00000,
    /// Trivial: interval is empty
    Trv = 0b_00100,
    /// Defined: expression is defined for all input values, but is discontinuous
    Def = 0b_01000,
    /// Defined & continuous
    ///
    /// Expression is defined and continuous for all input values, but either input or output of the function was unbounded
    Dac = 0b_01100,
    /// Common
    ///
    /// Expression is defined, continuous at all input points, and bounded, while its inputs are also bounded.
    Com = 0b_10000,
}

impl Decoration {
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        #[expect(clippy::as_conversions, reason = "comparison of ordered enum values")]
        if self as u8 <= other as u8 {
            self
        } else {
            other
        }
    }

    #[must_use]
    pub const fn min_if(self, other: Self, condition: bool) -> Self {
        if condition { self.min(other) } else { self }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DInterval {
    pub i: Interval,
    pub d: Decoration,
}

impl From<Interval> for DInterval {
    fn from(value: Interval) -> Self {
        Self {
            i: value,
            d: if value.is_nai() {
                Decoration::Ill
            } else if value.is_empty() {
                Decoration::Trv
            } else if value.is_infinite() {
                Decoration::Def
            } else {
                Decoration::Com
            },
        }
    }
}

impl From<DInterval> for Interval {
    fn from(value: DInterval) -> Self {
        value.i
    }
}

macro_rules! impl_decorated {
    (@com_consts $($CONST:ident)*) => {
        $( pub const $CONST: Self = Self { i: Interval::$CONST, d: Decoration::Com, }; )*
    };

    // @unwrap_interval: use value as `value.i` or bare `value`
    (@unwrap_interval $value:ident : Self) => { $value.i };
    (@unwrap_interval $value:ident : $any:tt) => { $value };

    // min decoration base case
    (@min_decoration $value:expr) => { $value };
    (@min_decoration $value:expr, $other:ident : Self $(, $($rest:tt)*)?) => {
        impl_decorated!(@min_decoration $value.min($other.d) $(, $($rest)*)?)
    };
    (@min_decoration $value:expr, $other:ident : $any:ty $(, $($rest:tt)*)?) => {
        impl_decorated!(@min_decoration $value $(, $($rest)*)?)
    };

    // returned type is Self
    ($(($constness:ident))? fn $method:ident(self $(,$arg:ident : $type:tt)*) -> Self) => {
        #[must_use]
        pub $($constness)? fn $method(self $(,$arg: $type)*) -> Self {
            Self {
                i: self.i.$method($(impl_decorated!(@unwrap_interval $arg : $type)),*),
                d: impl_decorated!(@min_decoration self.d  $(,$arg: $type)*)
            }
        }
    };
    // returned type is Self
    (@trv $(($constness:ident))? fn $method:ident(self $(,$arg:ident : $type:tt)*) -> Self) => {
        #[must_use]
        pub $($constness)? fn $method(self $(,$arg: $type)*) -> Self {
            Self {
                i: self.i.$method($(impl_decorated!(@unwrap_interval $arg : $type)),*),
                d: Decoration::Trv
            }
        }
    };
    // returned type is (Self, Self)
    ($(($constness:ident))? fn $method:ident(self $(,$arg:ident : $type:tt)*) -> (Self, Self)) => {
        #[must_use]
        pub $($constness)? fn $method(self $(,$arg: $type)*) -> (Self, Self) {
            let intervals =  self.i.$method($(impl_decorated!(@unwrap_interval $arg : $type)),*);
            let decoration = impl_decorated!(@min_decoration self.d  $(,$arg: $type)*);
            (
                Self { i: intervals.0, d: decoration },
                Self { i: intervals.1, d: decoration },
            )
        }
    };
    (@trv $(($constness:ident))? fn $method:ident(self $(,$arg:ident : $type:tt)*) -> (Self, Self)) => {
        #[must_use]
        pub $($constness)? fn $method(self $(,$arg: $type)*) -> (Self, Self) {
            let intervals =  self.i.$method($(impl_decorated!(@unwrap_interval $arg : $type)),*);
            let decoration = Decoration::Trv;
            (
                Self { i: intervals.0, d: decoration },
                Self { i: intervals.1, d: decoration },
            )
        }
    };
    // returned type is not Self
    ($(($constness:ident))? fn $method:ident(self $(,$arg:ident : $type:tt)*) -> $ret:ty) => {
        #[must_use]
        pub $($constness)? fn $method(self $(,$arg: $type)*) -> $ret {
            self.i.$method($(impl_decorated!(@unwrap_interval $arg : $type)),*)
        }
    };
}

impl DInterval {
    #[must_use]
    pub const fn new(inf: f64, sup: f64) -> Self {
        #![expect(clippy::float_cmp, reason = "handling equal infinite endpoints")]
        if inf.is_nan() || sup.is_nan() {
            Self::NAI
        } else if inf > sup || (inf == sup && inf.is_infinite()) {
            Self::EMPTY
        } else if inf.is_infinite() || sup.is_infinite() {
            Self {
                i: Interval(inf, sup),
                d: Decoration::Dac,
            }
        } else {
            Self {
                i: Interval(inf, sup),
                d: Decoration::Com,
            }
        }
    }
    #[must_use]
    pub const fn new_unordered(a: f64, b: f64) -> Self {
        if a <= b {
            Self::new(a, b)
        } else {
            Self::new(b, a)
        }
    }
    #[must_use]
    pub const fn new_singleton(p: f64) -> Self {
        Self::new(p, p)
    }
    #[must_use]
    pub const fn from_mid_rad(mid: f64, rad: f64) -> Self {
        if mid.is_nan() || rad.is_nan() || mid.is_infinite() {
            Self::NAI
        } else if rad < 0. {
            Self::EMPTY
        } else {
            Self::new(sub_rd(mid, rad), add_ru(mid, rad))
        }
    }
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

    pub const EMPTY: Self = Self {
        i: Interval::EMPTY,
        d: Decoration::Trv,
    };
    pub const ENTIRE: Self = Self {
        i: Interval::ENTIRE,
        d: Decoration::Def,
    };
    pub const NAI: Self = Self {
        i: Interval::NAI,
        d: Decoration::Ill,
    };
    pub const POSITIVE: Self = Self {
        i: Interval::POSITIVE,
        d: Decoration::Def,
    };
    pub const NEGATIVE: Self = Self {
        i: Interval::NEGATIVE,
        d: Decoration::Def,
    };
    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }
    #[must_use]
    pub const fn entire() -> Self {
        Self::ENTIRE
    }
    #[must_use]
    pub const fn nai() -> Self {
        Self::NAI
    }

    impl_decorated!(@com_consts ZERO ONE E FRAC_1_PI FRAC_1_SQRT_2 FRAC_2_PI FRAC_2_SQRT_PI FRAC_PI_2 FRAC_PI_3 FRAC_PI_4 FRAC_PI_6 FRAC_PI_8 LN_10 LN_2 LOG10_2 LOG10_E LOG2_10 LOG2_E PI SQRT_2 TAU);

    impl_decorated!((const) fn is_empty(self) -> bool);
    impl_decorated!((const) fn is_nai(self) -> bool);
    impl_decorated!((const) fn is_entire(self) -> bool);
    impl_decorated!((const) fn is_finite(self) -> bool);
    impl_decorated!((const) fn is_infinite(self) -> bool);
    impl_decorated!((const) fn inter(self, other: Self) -> Self);
    impl_decorated!((const) fn hull(self, other: Self) -> Self);
    impl_decorated!((const) fn next_out(self) -> Self);
    impl_decorated!(fn next_n(self, n_inf: i32, n_sup: i32) -> Self);
    impl_decorated!((const) fn set_difference_hull(self, other: Self) -> Self);
    impl_decorated!((const) fn set_difference_pair(self, other: Self) -> (Self, Self));
    impl_decorated!((const) fn compliment_pair(self) -> (Self, Self));
    impl_decorated!((const) fn symmetric_difference_pair(self, other: Self) -> (Self, Self));

    impl_decorated!((const) fn contains(self, value: f64) -> bool);
    impl_decorated!((const) fn interior_contains(self, value: f64) -> bool);
    impl_decorated!((const) fn overlap(self, other: Self) -> Overlap);
    impl_decorated!((const) fn equal(self, other: Self) -> bool);
    impl_decorated!((const) fn le_weak(self, other: Self) -> bool);
    impl_decorated!((const) fn ge_weak(self, other: Self) -> bool);
    impl_decorated!((const) fn le_all(self, other: Self) -> bool);
    impl_decorated!((const) fn ge_all(self, other: Self) -> bool);
    impl_decorated!((const) fn lt_weak(self, other: Self) -> bool);
    impl_decorated!((const) fn gt_weak(self, other: Self) -> bool);
    impl_decorated!((const) fn lt_all(self, other: Self) -> bool);
    impl_decorated!((const) fn gt_all(self, other: Self) -> bool);
    impl_decorated!((const) fn subset(self, other: Self) -> bool);
    impl_decorated!((const) fn interior(self, other: Self) -> bool);
    impl_decorated!((const) fn proper_subset(self, other: Self) -> bool);
    impl_decorated!((const) fn supset(self, other: Self) -> bool);
    impl_decorated!((const) fn exterior(self, other: Self) -> bool);
    impl_decorated!((const) fn proper_supset(self, other: Self) -> bool);
    impl_decorated!((const) fn disjoint(self, other: Self) -> bool);

    impl_decorated!((const) fn neg(self) -> Self);
    impl_decorated!((const) fn add(self, rhs: Self) -> Self);
    impl_decorated!((const) fn sub(self, rhs: Self) -> Self);
    impl_decorated!((const) fn mul(self, rhs: Self) -> Self);
    #[must_use]
    pub const fn div(self, rhs: Self) -> Self {
        let i = self.i.div(rhs.i);
        let d = self
            .d
            .min(rhs.d)
            .min_if(Decoration::Dac, rhs.i.contains(0.))
            .min_if(Decoration::Def, rhs.i.interior_contains(0.));
        Self { i, d }
    }
    #[must_use]
    pub const fn rem(self, rhs: Self) -> Self {
        self.div(rhs).fract().mul(rhs)
    }

    #[must_use]
    pub const fn recip(self) -> Self {
        Self {
            i: self.i.recip(),
            d: (self.d)
                .min_if(Decoration::Ill, self.i.equal(Interval::ZERO))
                .min_if(Decoration::Dac, self.i.contains(0.))
                .min_if(Decoration::Def, self.i.interior_contains(0.)),
        }
    }
    impl_decorated!((const) fn sqr(self) -> Self);
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self {
            i: self.i.sqrt(),
            d: self.d.min_if(Decoration::Ill, self.i.0 < 0.),
        }
    }
    impl_decorated!(fn cbrt(self) -> Self);
    impl_decorated!(fn mul_add(self, x: Self, y: Self) -> Self);
    impl_decorated!(fn hypot(self, other: Self) -> Self);
    #[must_use]
    pub fn div_euclid(self, other: Self) -> Self {
        Self::hull(
            self.div(other.inter(Self::POSITIVE)).floor(),
            self.div(other.inter(Self::NEGATIVE)).ceil(),
        )
    }

    impl_decorated!(fn powi(self, n: i32) -> Self);
    // impl_decorated!(fn pow(self, p: Self) -> Self);
    impl_decorated!(fn exp(self) -> Self);
    impl_decorated!(fn exp2(self) -> Self);
    impl_decorated!(fn exp10(self) -> Self);
    impl_decorated!(fn exp_m1(self) -> Self);
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }
    #[must_use]
    pub fn ln(self) -> Self {
        Self {
            i: self.i.ln(),
            d: self.d.min_if(Decoration::Ill, self.i.0 <= 0.),
        }
    }
    #[must_use]
    pub fn log2(self) -> Self {
        Self {
            i: self.i.log2(),
            d: self.d.min_if(Decoration::Ill, self.i.0 <= 0.),
        }
    }
    #[must_use]
    pub fn log10(self) -> Self {
        Self {
            i: self.i.log10(),
            d: self.d.min_if(Decoration::Ill, self.i.0 <= 0.),
        }
    }
    #[must_use]
    pub fn ln_1p(self) -> Self {
        Self {
            i: self.i.ln_1p(),
            d: self.d.min_if(Decoration::Ill, self.i.0 <= -1.),
        }
    }

    impl_decorated!(fn sin(self) -> Self);
    impl_decorated!(fn sin_pi(self) -> Self);
    impl_decorated!(fn cos(self) -> Self);
    impl_decorated!(fn cos_pi(self) -> Self);
    impl_decorated!(fn sin_cos(self) -> (Self, Self));
    #[must_use]
    pub fn tan(self) -> Self {
        let i = self.i.tan();
        let d = self.d.min_if(Decoration::Def, i.is_infinite());
        Self { i, d }
    }
    #[must_use]
    pub fn tan_pi(self) -> Self {
        let i = self.i.tan_pi();
        let d = self.d.min_if(Decoration::Def, i.is_infinite());
        Self { i, d }
    }
    #[must_use]
    pub fn asin(self) -> Self {
        let i = self.i.asin();
        Self {
            i,
            d: self.d.min_if(Decoration::Ill, i.is_nai()),
        }
    }
    #[must_use]
    pub fn asin_pi(self) -> Self {
        let i = self.i.asin_pi();
        Self {
            i,
            d: self.d.min_if(Decoration::Ill, i.is_nai()),
        }
    }
    #[must_use]
    pub fn acos(self) -> Self {
        let i = self.i.acos();
        Self {
            i,
            d: self.d.min_if(Decoration::Ill, i.is_nai()),
        }
    }
    #[must_use]
    pub fn acos_pi(self) -> Self {
        let i = self.i.acos_pi();
        Self {
            i,
            d: self.d.min_if(Decoration::Ill, i.is_nai()),
        }
    }
    impl_decorated!(fn atan(self) -> Self);
    impl_decorated!(fn atan_pi(self) -> Self);
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        let i = self.i.atan2(other.i);
        let d = self
            .d
            .min(other.d)
            .min_if(Decoration::Ill, self.i.contains(0.) && other.i.contains(0.))
            .min_if(
                Decoration::Def,
                self.i.interior_contains(0.) && other.i.1 < 0.,
            );
        Self { i, d }
    }
    #[must_use]
    pub fn atan2_pi(self, other: Self) -> Self {
        let i = self.i.atan2_pi(other.i);
        let d = self
            .d
            .min(other.d)
            .min_if(Decoration::Ill, self.i.contains(0.) && other.i.contains(0.))
            .min_if(
                Decoration::Def,
                self.i.interior_contains(0.) && other.i.1 < 0.,
            );
        Self { i, d }
    }
    impl_decorated!(fn sinh(self) -> Self);
    impl_decorated!(fn cosh(self) -> Self);
    impl_decorated!(fn tanh(self) -> Self);
    impl_decorated!(fn asinh(self) -> Self);
    #[must_use]
    pub fn acosh(self) -> Self {
        Self {
            i: self.i.acosh(),
            d: self.d.min_if(Decoration::Ill, self.i.0 < 1.),
        }
    }
    impl_decorated!(fn atanh(self) -> Self);
    #[must_use]
    pub const fn signum(self) -> Self {
        Self {
            i: self.i.signum(),
            d: self.d.min_if(Decoration::Def, self.contains(0.)),
        }
    }
    #[must_use]
    pub const fn sign(self) -> Self {
        Self {
            i: self.i.sign(),
            d: self.d.min_if(Decoration::Def, self.contains(0.)),
        }
    }
    #[must_use]
    pub const fn ceil(self) -> Self {
        let i = self.i.ceil();
        let d = self.d.min_if(Decoration::Def, !i.is_singleton());
        Self { i, d }
    }
    #[must_use]
    pub const fn floor(self) -> Self {
        let i = self.i.floor();
        let d = self.d.min_if(Decoration::Def, !i.is_singleton());
        Self { i, d }
    }
    #[must_use]
    pub const fn trunc(self) -> Self {
        let i = self.i.trunc();
        let d = self.d.min_if(Decoration::Def, !i.is_singleton());
        Self { i, d }
    }
    #[must_use]
    pub const fn round(self) -> Self {
        let i = self.i.round();
        let d = self.d.min_if(Decoration::Def, !i.is_singleton());
        Self { i, d }
    }
    #[must_use]
    pub const fn round_ties_even(self) -> Self {
        let i = self.i.round_ties_even();
        let d = self.d.min_if(Decoration::Def, !i.is_singleton());
        Self { i, d }
    }
    #[must_use]
    pub const fn fract(self) -> Self {
        Self {
            i: self.i.fract(),
            d: self
                .d
                .min_if(Decoration::Def, !self.i.trunc().is_singleton()),
        }
    }
    impl_decorated!((const) fn abs(self) -> Self);
    impl_decorated!((const) fn min(self, other: Self) -> Self);
    impl_decorated!((const) fn max(self, other: Self) -> Self);
    impl_decorated!((const) fn abs_sub(self, other: Self) -> Self);
    #[must_use]
    pub const fn cancel_add(self, other: Self) -> Self {
        let i = self.i.cancel_add(other.i);
        let d = (self.d).min(other.d).min_if(Decoration::Trv, i.is_empty());
        Self { i, d }
    }
    #[must_use]
    pub const fn cancel_sub(self, other: Self) -> Self {
        let i = self.i.cancel_sub(other.i);
        let d = (self.d).min(other.d).min_if(Decoration::Trv, i.is_empty());
        Self { i, d }
    }
    impl_decorated!(@trv fn sqr_rev(self, domain: Self) -> Self);
    impl_decorated!(@trv (const) fn abs_rev(self, domain: Self) -> Self);
    impl_decorated!(@trv fn powi_rev(self, n: i32, domain: Self) -> Self);
    impl_decorated!(@trv fn sin_rev(self, domain: Self) -> Self);
    impl_decorated!(@trv fn cos_rev(self, domain: Self) -> Self);
    impl_decorated!(@trv fn tan_rev(self, domain: Self) -> Self);
    impl_decorated!(@trv fn cosh_rev(self, domain: Self) -> Self);
    impl_decorated!(@trv fn mul_rev(self, rhs: Self, domain: Self) -> Self);
    impl_decorated!(@trv fn pow_rev1(self, power: Self, domain: Self) -> Self);
    impl_decorated!(@trv fn pow_rev2(self, base: Self, domain: Self) -> Self);
    impl_decorated!(@trv fn atan2_rev1(self, x: Self, domain: Self) -> Self);
    impl_decorated!(@trv fn atan2_rev2(self, y: Self, domain: Self) -> Self);
    impl_decorated!(@trv (const) fn mul_rev_to_pair(self, rhs: Self) -> (Self, Self));
}

impl Neg for DInterval {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.neg()
    }
}
impl Add for DInterval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}
impl Sub for DInterval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}
impl Mul for DInterval {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}
impl Div for DInterval {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}
impl Rem for DInterval {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.rem(rhs)
    }
}

macro_rules! forward_dinterval_binop {
    (impl $imp:ident::$method:ident for $lhs:ident, $rhs:ident ) => {
        forward_dinterval_binop!(@impl $imp::$method for $lhs, $rhs);
        forward_dinterval_binop!(@impl $imp::$method for $rhs, $lhs);
    };
    (@impl $imp:ident::$method:ident for $lhs:ident, $rhs:ident ) => {
        impl $imp<$rhs> for $lhs {
            type Output = DInterval;

            #[inline]
            fn $method(self, other: $rhs) -> Self::Output {
                Into::<DInterval>::into(self).$method(Into::<DInterval>::into(other))
            }
        }
    };
}

forward_dinterval_binop!(impl Add::add for Interval, DInterval);
forward_dinterval_binop!(impl Sub::sub for Interval, DInterval);
forward_dinterval_binop!(impl Mul::mul for Interval, DInterval);
forward_dinterval_binop!(impl Div::div for Interval, DInterval);
forward_dinterval_binop!(impl Rem::rem for Interval, DInterval);
