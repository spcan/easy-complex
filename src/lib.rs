//! easy-complex crate
//!
//! Crate for easy to use complex numbers


pub(crate) mod common {
	#[cfg(std)]
	pub use std::*;

	#[cfg(not(std))]
	extern crate core;

	#[cfg(not(std))]
	pub use self::core::*;
}

#[macro_use]
pub mod methods;

#[macro_use]
pub mod convert;

#[macro_use]
pub mod display;

#[macro_use]
pub mod ops;

#[macro_use]
pub mod angle;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
	InvalidRootValue,
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

pub type EComplex32 = EComplex<f32>;
pub type EComplex64 = EComplex<f64>;

pub use self::angle::Angle;
impl_angle!([f32, f64]);

/// `Complex` number represented in rectangular coordinates
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Complex<T> {
	pub real: T,
	pub imag: T,
}

/// `EComplex` number represented in exponential form
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EComplex<T> {
	pub module: T,
	pub arg: T,
}

basic!(Complex,  [f32, f64]);
basic!(EComplex, [f32, f64]);

convert!([i8, u8, i16, u16, f32], f32, Complex);
convert!([i8, u8, i16, u16, f32], f32, EComplex);
convert!([i8, u8, i16, u16, f32, i32, u32, f64], f64, Complex);
convert!([i8, u8, i16, u16, f32, i32, u32, f64], f64, EComplex);

dual_convert!(Complex, EComplex);

gen_display!(Complex,  [f32, f64]);
gen_display!(EComplex, [f32, f64]);

gen_ops!(f32, Complex);
gen_ops!(f64, Complex);
gen_ops!(f32, EComplex);
gen_ops!(f64, EComplex);


math!(Complex,   [f32, f64]);
math!(EComplex,  [f32, f64]);

geometry!(Complex,  [f32, f64]);
geometry!(EComplex, [f32, f64]);