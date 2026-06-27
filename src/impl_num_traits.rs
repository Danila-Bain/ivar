use core::num::FpCategory;

use num_traits::{
    Bounded, ConstOne, ConstZero, Float, FloatConst, FromPrimitive, Num, NumCast, One, Signed,
    ToPrimitive, Zero, float::FloatCore,
};

use crate::Interval;

impl ConstZero for Interval {
    const ZERO: Self = Self::ZERO;
}
impl ConstOne for Interval {
    const ONE: Self = Self::ONE;
}
impl Zero for Interval {
    fn zero() -> Self {
        Self::ZERO
    }
    fn is_zero(&self) -> bool {
        self.0 == 0. && self.1 == 0.
    }
}
impl One for Interval {
    fn one() -> Self {
        Self::ONE
    }
}

impl Bounded for Interval {
    fn min_value() -> Self {
        Self::new_singleton(<f64 as Bounded>::min_value())
    }
    fn max_value() -> Self {
        Self::new_singleton(<f64 as Bounded>::max_value())
    }
}

impl Signed for Interval {
    fn abs(&self) -> Self {
        Interval::abs(*self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Interval::abs_sub(*self, *other)
    }

    fn signum(&self) -> Self {
        Interval::signum(*self)
    }

    fn is_positive(&self) -> bool {
        self.0.is_positive() && self.1.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative() && self.1.is_negative()
    }
}

/// Forward `FromPrimitive` methods
macro_rules! impl_from_primitive {
    ($method:ident, $type:ident) => {
        fn $method(n: $type) -> Option<Self> {
            <f64 as FromPrimitive>::$method(n).map(Self::new_singleton)
        }
    };
}
impl FromPrimitive for Interval {
    impl_from_primitive!(from_i64, i64);
    impl_from_primitive!(from_u64, u64);
    impl_from_primitive!(from_isize, isize);
    impl_from_primitive!(from_i8, i8);
    impl_from_primitive!(from_i16, i16);
    impl_from_primitive!(from_i32, i32);
    impl_from_primitive!(from_i128, i128);
    impl_from_primitive!(from_usize, usize);
    impl_from_primitive!(from_u8, u8);
    impl_from_primitive!(from_u16, u16);
    impl_from_primitive!(from_u32, u32);
    impl_from_primitive!(from_u128, u128);
    impl_from_primitive!(from_f32, f32);
    impl_from_primitive!(from_f64, f64);
}

/// Forward `ToPrimitive` method implementation to `self.mid()`.
macro_rules! impl_to_primitive {
    ($method:ident, $type:ident) => {
        fn $method(&self) -> Option<$type> {
            self.mid().$method()
        }
    };
}

impl ToPrimitive for Interval {
    impl_to_primitive!(to_i64, i64);
    impl_to_primitive!(to_u64, u64);
    impl_to_primitive!(to_isize, isize);
    impl_to_primitive!(to_i8, i8);
    impl_to_primitive!(to_i16, i16);
    impl_to_primitive!(to_i32, i32);
    impl_to_primitive!(to_i128, i128);
    impl_to_primitive!(to_usize, usize);
    impl_to_primitive!(to_u8, u8);
    impl_to_primitive!(to_u16, u16);
    impl_to_primitive!(to_u32, u32);
    impl_to_primitive!(to_u128, u128);
    impl_to_primitive!(to_f32, f32);
    impl_to_primitive!(to_f64, f64);
}

impl Num for Interval {
    type FromStrRadixErr = <f64 as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self::new_singleton(f64::from_str_radix(str, radix)?))
    }
}

impl NumCast for Interval {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f64().map(Self::new_singleton)
    }
}

/// Forward `Float` method implementation to corresponding method of `Interval`.
macro_rules! impl_float_method {
    (@constant $method:ident) => {
        fn $method() -> Self {
            Self::new_singleton(<f64 as Float>::$method())
        }
    };
    (@unary $method:ident) => {
        fn $method(self) -> Self {
            self.$method()
        }
    };
}

impl Float for Interval {
    impl_float_method!(@constant nan);
    impl_float_method!(@constant infinity);
    impl_float_method!(@constant neg_infinity);
    impl_float_method!(@constant neg_zero);
    impl_float_method!(@constant min_value);
    impl_float_method!(@constant min_positive_value);
    impl_float_method!(@constant max_value);

    fn is_nan(self) -> bool {
        self.is_nai()
    }

    fn is_infinite(self) -> bool {
        self.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn is_normal(self) -> bool {
        self.0.is_normal() && self.1.is_normal()
    }

    fn classify(self) -> FpCategory {
        match (self.0.classify(), self.1.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => FpCategory::Nan,
            (FpCategory::Infinite, _) | (_, FpCategory::Infinite) => FpCategory::Infinite,
            (FpCategory::Subnormal, _) | (_, FpCategory::Subnormal) => FpCategory::Subnormal,
            (FpCategory::Zero, FpCategory::Zero) => FpCategory::Zero,
            _ => FpCategory::Normal,
        }
    }

    impl_float_method!(@unary floor);
    impl_float_method!(@unary ceil);
    impl_float_method!(@unary round);
    impl_float_method!(@unary trunc);
    impl_float_method!(@unary fract);
    impl_float_method!(@unary abs);
    impl_float_method!(@unary signum);

    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive() && self.1.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative() && self.1.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    impl_float_method!(@unary recip);

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn powf(self, n: Self) -> Self {
        self.pow(n)
    }

    impl_float_method!(@unary sqrt);
    impl_float_method!(@unary exp);
    impl_float_method!(@unary exp2);
    fn log(self, base: Self) -> Self {
        self.log(base)
    }
    impl_float_method!(@unary ln);
    impl_float_method!(@unary log2);
    impl_float_method!(@unary log10);
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn abs_sub(self, other: Self) -> Self {
        self.abs_sub(other)
    }
    impl_float_method!(@unary cbrt);
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }
    impl_float_method!(@unary sin);
    impl_float_method!(@unary cos);
    impl_float_method!(@unary tan);
    impl_float_method!(@unary asin);
    impl_float_method!(@unary acos);
    impl_float_method!(@unary atan);
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    impl_float_method!(@unary exp_m1);
    impl_float_method!(@unary ln_1p);
    impl_float_method!(@unary sinh);
    impl_float_method!(@unary cosh);
    impl_float_method!(@unary tanh);
    impl_float_method!(@unary asinh);
    impl_float_method!(@unary acosh);
    impl_float_method!(@unary atanh);
    fn integer_decode(self) -> (u64, i16, i8) {
        <f64 as Float>::integer_decode(self.mid())
    }
}

/// Implement `FloatConst` constant by forwarding to the corresponding `Interval` constant.
macro_rules! forward_constants {
    ($c:ident) => {
        fn $c() -> Self {
            Self::$c
        }
    };
}

impl FloatConst for Interval {
    forward_constants!(E);
    forward_constants!(FRAC_1_PI);
    forward_constants!(FRAC_1_SQRT_2);
    forward_constants!(FRAC_2_PI);
    forward_constants!(FRAC_2_SQRT_PI);
    forward_constants!(FRAC_PI_2);
    forward_constants!(FRAC_PI_3);
    forward_constants!(FRAC_PI_4);
    forward_constants!(FRAC_PI_6);
    forward_constants!(FRAC_PI_8);
    forward_constants!(LN_10);
    forward_constants!(LN_2);
    forward_constants!(LOG10_E);
    forward_constants!(LOG2_E);
    forward_constants!(PI);
    forward_constants!(SQRT_2);
}
/// Forward `FloatCore` method implementation to corresponding method of `Interval`.
macro_rules! impl_float_core_method {
    (@constant $method:ident) => {
        fn $method() -> Self {
            Self::new_singleton(<f64 as FloatCore>::$method())
        }
    };
}

impl FloatCore for Interval {
    impl_float_core_method!(@constant infinity);
    impl_float_core_method!(@constant neg_infinity);
    impl_float_core_method!(@constant nan);
    impl_float_core_method!(@constant neg_zero);
    impl_float_core_method!(@constant min_value);
    impl_float_core_method!(@constant min_positive_value);
    impl_float_core_method!(@constant epsilon);
    impl_float_core_method!(@constant max_value);

    fn classify(self) -> FpCategory {
        match (self.0.classify(), self.1.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => FpCategory::Nan,
            (FpCategory::Infinite, _) | (_, FpCategory::Infinite) => FpCategory::Infinite,
            (FpCategory::Subnormal, _) | (_, FpCategory::Subnormal) => FpCategory::Subnormal,
            (FpCategory::Zero, FpCategory::Zero) => FpCategory::Zero,
            _ => FpCategory::Normal,
        }
    }

    fn to_degrees(self) -> Self {
        self / Self::PI * 180.
    }

    fn to_radians(self) -> Self {
        self / 180. * Self::PI
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        <f64 as FloatCore>::integer_decode(self.mid())
    }
}
