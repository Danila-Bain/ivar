use num_complex::Complex;

use crate::Interval;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComplexBox(Complex<Interval>);

macro_rules! forward_complex_method {
    //
    // Methods that return self
    //
    (fn $method:ident($($param:ident: $type:ty),*) -> Self) => {
        #[must_use]
        pub fn $method($($param : $type),*) -> Self {
            Self(Complex::$method( $($param),* ))
        }
    };
    (fn $method:ident(&self $(, $param:ident : $type:ty)*) -> Self) => {
        #[must_use]
        pub fn $method(&self, $($param : $type),*) -> Self {
            Self(self.0.$method( $($param),* ))
        }
    };
    (fn $method:ident(self $(, $param:ident : Self)*) -> Self) => {
        #[must_use]
        pub fn $method(self, $($param : Self),*) -> Self {
            Self(self.0.$method( $($param.0),* ))
        }
    };
    (fn $method:ident(self $(, $param:ident : $type:ty)*) -> Self) => {
        #[must_use]
        pub fn $method(self, $($param : $type),*) -> Self {
            Self(self.0.$method( $($param),* ))
        }
    };
    //
    // Methods that return Interval or pair of intervals
    //
    (fn $method:ident(self $(, $param:ident : $type:ty)*) -> $ret:ty) => {
        #[must_use]
        pub fn $method(self, $($param : $type),*) -> $ret {
            self.0.$method( $($param),* )
        }
    };
    (fn $method:ident(&self $(, $param:ident : $type:ty)*) -> $ret:ty) => {
        #[must_use]
        pub fn $method(&self, $($param : $type),*) -> $ret {
            self.0.$method( $($param),* )
        }
    };
}

impl ComplexBox {
    #[must_use]
    pub const fn new(re: Interval, im: Interval) -> Self {
        Self(Complex::new(re, im))
    }
    forward_complex_method! {fn i() -> Self }
    forward_complex_method! {fn norm_sqr(&self) -> Interval }
    forward_complex_method! {fn scale(&self, t: Interval) -> Self }
    forward_complex_method! {fn unscale(&self, t: Interval) -> Self }
    forward_complex_method! {fn powu(&self, exp: u32) -> Self }
    forward_complex_method! {fn conj(&self) -> Self }
    forward_complex_method! {fn inv(&self) -> Self }
    forward_complex_method! {fn powi(&self, exp: i32) -> Self }
    forward_complex_method! {fn l1_norm(&self) -> Interval }
    forward_complex_method! {fn cis(phase: Interval) -> Self }
    forward_complex_method! {fn norm(self) -> Interval }
    forward_complex_method! {fn arg(self) -> Interval }
    forward_complex_method! {fn to_polar(self) -> (Interval, Interval) }
    forward_complex_method! {fn from_polar(r: Interval, theta: Interval) -> Self }
    forward_complex_method! {fn exp(self) -> Self }
    forward_complex_method! {fn ln(self) -> Self }
    forward_complex_method! {fn sqrt(self) -> Self }
    forward_complex_method! {fn cbrt(self) -> Self }
    forward_complex_method! {fn powf(self, exp: Interval) -> Self }
    forward_complex_method! {fn log(self, base: Interval) -> Self }
    forward_complex_method! {fn powc(self, exp: Self) -> Self }
    forward_complex_method! {fn expf(self, base: Interval) -> Self }
    forward_complex_method! {fn sin(self) -> Self }
    forward_complex_method! {fn cos(self) -> Self }
    forward_complex_method! {fn tan(self) -> Self }
    forward_complex_method! {fn asin(self) -> Self }
    forward_complex_method! {fn acos(self) -> Self }
    forward_complex_method! {fn atan(self) -> Self }
    forward_complex_method! {fn sinh(self) -> Self }
    forward_complex_method! {fn cosh(self) -> Self }
    forward_complex_method! {fn tanh(self) -> Self }
    forward_complex_method! {fn asinh(self) -> Self }
    forward_complex_method! {fn acosh(self) -> Self }
    forward_complex_method! {fn atanh(self) -> Self }
    forward_complex_method! {fn finv(self) -> Self }
    forward_complex_method! {fn fdiv(self, other: Self) -> Self }
    forward_complex_method! {fn exp2(self) -> Self }
    forward_complex_method! {fn log2(self) -> Self }
    forward_complex_method! {fn log10(self) -> Self }
    forward_complex_method! {fn is_nan(self) -> bool }
    forward_complex_method! {fn is_infinite(self) -> bool }
    forward_complex_method! {fn is_finite(self) -> bool }
    forward_complex_method! {fn is_normal(self) -> bool }

    //     fn from(re: T) -> Self {
    //     fn from(re: &T) -> Self {
    //             fn $method(self, other: &Complex<T>) -> Self::Output {
    //             fn $method(self, other: Complex<T>) -> Self::Output {
    //             fn $method(self, other: &Complex<T>) -> Self::Output {
    //     fn add(self, other: Self) -> Self::Output {
    //     fn sub(self, other: Self) -> Self::Output {
    //     fn mul(self, other: Self) -> Self::Output {
    //     fn mul_add(self, other: Complex<T>, add: Complex<T>) -> Complex<T> {
    //     fn mul_add(self, other: &Complex<T>, add: &Complex<T>) -> Complex<T> {
    //     fn div(self, other: Self) -> Self::Output {
    //     fn div_trunc(&self, divisor: &Self) -> Self {
    //     fn rem(self, modulus: Self) -> Self::Output {
    //         fn add_assign(&mut self, other: Self) {
    //         fn sub_assign(&mut self, other: Self) {
    //         fn mul_assign(&mut self, other: Self) {
    //         fn mul_add_assign(&mut self, other: Complex<T>, add: Complex<T>) {
    //         fn mul_add_assign(&mut self, other: &Complex<T>, add: &Complex<T>) {
    //         fn div_assign(&mut self, other: Self) {
    //         fn rem_assign(&mut self, modulus: Self) {
    //         fn add_assign(&mut self, other: T) {
    //         fn sub_assign(&mut self, other: T) {
    //         fn mul_assign(&mut self, other: T) {
    //         fn div_assign(&mut self, other: T) {
    //         fn rem_assign(&mut self, other: T) {
    //                 fn $method(&mut self, other: &Self) {
    //                 fn $method(&mut self, other: &T) {
    //     fn neg(self) -> Self::Output {
    //     fn neg(self) -> Self::Output {
    //     fn inv(self) -> Self::Output {
    //     fn inv(self) -> Self::Output {
    //             fn $method(self, other: &T) -> Self::Output {
    //             fn $method(self, other: T) -> Self::Output {
    //             fn $method(self, other: &T) -> Self::Output {
    //                 fn $method(self, other: &Complex<$real>) -> Complex<$real> {
    //                 fn $method(self, other: Complex<$real>) -> Complex<$real> {
    //                 fn $method(self, other: &Complex<$real>) -> Complex<$real> {
    //                 fn add(self, other: Complex<$real>) -> Self::Output {
    //                 fn sub(self, other: Complex<$real>) -> Self::Output  {
    //                 fn mul(self, other: Complex<$real>) -> Self::Output {
    //                 fn div(self, other: Complex<$real>) -> Self::Output {
    //                 fn rem(self, other: Complex<$real>) -> Self::Output {
    //     fn add(self, other: T) -> Self::Output {
    //     fn sub(self, other: T) -> Self::Output {
    //     fn mul(self, other: T) -> Self::Output {
    //     fn div(self, other: T) -> Self::Output {
    //     fn rem(self, other: T) -> Self::Output {
    //     fn zero() -> Self {
    //     fn is_zero(&self) -> bool {
    //     fn set_zero(&mut self) {
    //     fn one() -> Self {
    //     fn is_one(&self) -> bool {
    //     fn set_one(&mut self) {
    //         fn fmt_re_im(
    //         fn fmt_complex(f: &mut fmt::Formatter<'_>, complex: fmt::Arguments<'_>) -> fmt::Result {
    //         fn fmt_complex(f: &mut fmt::Formatter<'_>, complex: fmt::Arguments<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // fn from_str_generic<T, E, F>(s: &str, from: F) -> Result<Complex<T>, ParseComplexError<E>>
    //     F: Fn(&str) -> Result<T, E>,
    //     fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
    //     fn sum<I>(iter: I) -> Self
    //     fn sum<I>(iter: I) -> Self
    //     fn product<I>(iter: I) -> Self
    //     fn product<I>(iter: I) -> Self
    //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //     fn expr_error() -> Self {
    //     fn unsupported_radix() -> Self {
    //     fn from_error(error: E) -> Self {
    //     fn description(&self) -> &str {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // fn hash<T: hash::Hash>(x: &T) -> u64 {
    //     fn test_consts() {
    //         fn test(c: Complex64, r: f64, i: f64) {
    //     fn test_scale_unscale() {
    //     fn test_conj() {
    //     fn test_inv() {
    //     fn test_divide_by_zero_natural() {
    //     fn test_inv_zero() {
    //     fn test_l1_norm() {
    //     fn test_pow() {
    //         fn test_cis() {
    //         fn test_norm() {
    //             fn test(c: Complex64, ns: f64) {
    //         fn test_arg() {
    //             fn test(c: Complex64, arg: f64) {
    //         fn test_polar_conv() {
    //             fn test(c: Complex64) {
    //         pub(crate) fn close(a: Complex64, b: Complex64) -> bool {
    //         fn close_to_tol(a: Complex64, b: Complex64, tol: f64) -> bool {
    //         fn close_naninf(a: Complex64, b: Complex64) -> bool {
    //         fn close_naninf_to_tol(a: Complex64, b: Complex64, tol: f64) -> bool {
}
