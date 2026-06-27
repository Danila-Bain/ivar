#![allow(
    clippy::missing_docs_in_private_items,
    reason = "macro names are self-documentary"
)]
use crate::{DInterval, Interval};
use core::ops::{Add, Div, Mul, Rem, Sub};

macro_rules! into_owned_interval {
    ($value:ident: f64) => {
        Interval::new_singleton($value)
    };
    ($value:ident: &f64) => {
        Interval::new_singleton($value.clone())
    };
    ($value:ident: Interval) => {
        $value
    };
    ($value:ident: &Interval) => {
        $value.clone()
    };
    ($value:ident: DInterval) => {
        $value
    };
    ($value:ident: &DInterval) => {
        $value.clone()
    };
}

macro_rules! forward_ref_ref_binop {
    (impl $imp:ident::$method:ident -> $output:ident for $rhs:ident, $lhs:ident ) => {
        impl $imp<&$rhs> for &$lhs {
            type Output = $output;
            #[inline]
            fn $method(self, other: &$rhs) -> Self::Output {
                $imp::$method(into_owned_interval!(self: &$lhs), (into_owned_interval!(other: &$rhs)))
            }
        }
    };
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident::$method:ident -> $output:ident for $lhs:ident, $rhs:ident ) => {
        impl $imp<$rhs> for &$lhs {
            type Output = $output;

            #[inline]
            fn $method(self, other: $rhs) -> Self::Output {
                $imp::$method(into_owned_interval!(self: &$lhs), (into_owned_interval!(other: $rhs)))
            }
        }
    };
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident::$method:ident -> $output:ident for $lhs:ident, $rhs:ident ) => {
        impl $imp<&$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $method(self, other: &$rhs) -> Self::Output {
                $imp::$method(into_owned_interval!(self: $lhs), (into_owned_interval!(other: &$rhs)))
            }
        }
    };
}

macro_rules! forward_val_val_binop {
    (impl $imp:ident::$method:ident -> $output:ident for $lhs:ident, $rhs:ident ) => {
        impl $imp<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $method(self, other: $rhs) -> Self::Output {
                $imp::$method(into_owned_interval!(self: $lhs), (into_owned_interval!(other: $rhs)))
            }
        }
    };
}

macro_rules! forward_all_binop {
    (impl $imp:ident::$method:ident) => {
        forward_ref_ref_binop!(impl $imp::$method -> Interval for Interval, Interval);
        forward_ref_val_binop!(impl $imp::$method -> Interval for Interval, Interval);
        forward_val_ref_binop!(impl $imp::$method -> Interval for Interval, Interval);
        forward_ref_ref_binop!(impl $imp::$method -> Interval for f64, Interval);
        forward_ref_val_binop!(impl $imp::$method -> Interval for f64, Interval);
        forward_val_ref_binop!(impl $imp::$method -> Interval for f64, Interval);
        forward_val_val_binop!(impl $imp::$method -> Interval for f64, Interval);
        forward_ref_ref_binop!(impl $imp::$method -> Interval for Interval, f64);
        forward_ref_val_binop!(impl $imp::$method -> Interval for Interval, f64);
        forward_val_ref_binop!(impl $imp::$method -> Interval for Interval, f64);
        forward_val_val_binop!(impl $imp::$method -> Interval for Interval, f64);

        forward_ref_ref_binop!(impl $imp::$method -> DInterval for DInterval, Interval);
        forward_ref_val_binop!(impl $imp::$method -> DInterval for DInterval, Interval);
        forward_val_ref_binop!(impl $imp::$method -> DInterval for DInterval, Interval);
        forward_ref_ref_binop!(impl $imp::$method -> DInterval for Interval, DInterval);
        forward_ref_val_binop!(impl $imp::$method -> DInterval for Interval, DInterval);
        forward_val_ref_binop!(impl $imp::$method -> DInterval for Interval, DInterval);
        forward_ref_ref_binop!(impl $imp::$method -> DInterval for f64, DInterval);
        forward_ref_val_binop!(impl $imp::$method -> DInterval for f64, DInterval);
        forward_val_ref_binop!(impl $imp::$method -> DInterval for f64, DInterval);
        forward_val_val_binop!(impl $imp::$method -> DInterval for f64, DInterval);
        forward_ref_ref_binop!(impl $imp::$method -> DInterval for DInterval, f64);
        forward_ref_val_binop!(impl $imp::$method -> DInterval for DInterval, f64);
        forward_val_ref_binop!(impl $imp::$method -> DInterval for DInterval, f64);
        forward_val_val_binop!(impl $imp::$method -> DInterval for DInterval, f64);
    };
}

forward_all_binop!(impl Add::add);
forward_all_binop!(impl Sub::sub);
forward_all_binop!(impl Mul::mul);
forward_all_binop!(impl Div::div);
forward_all_binop!(impl Rem::rem);
