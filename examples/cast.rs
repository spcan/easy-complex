extern crate easy_complex;

use easy_complex::ContainedInComplex;
use easy_complex::{ExpComplex};

fn main() {
	//Explicit cast of a number
	println!("{}", 1.32f32.complex());

	//Explicit cast and direct operation
	println!("{}", 1.complex() + 2.complex());
	println!("{}", 1.complex() + ExpComplex{module: 2.0, arg: 0.0});
}
