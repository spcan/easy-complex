//!easy-complex crate
//!
//!Crate for easy to use complex numbers
//!Needs the std library
//!

use std::ops::{Add, Mul, Neg, Sub, Div};
use std::fmt;

#[cfg(feature="num_complex_compatible")]
extern crate num_complex;

#[cfg(feature="num_complex_compatible")]
use num_complex::Complex64;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_check() {
        assert!(Complex{real: 1.0, imag: 0.0} == EComplex{module: 1.0, arg: 0.0});
    }

    #[test]
    fn casting_check() {
        let cnum: Complex = 1.complex().into();
        let ecnum = 1.complex();
        assert!(cnum == ecnum);
    }

    #[test]
    fn hyperbolic_trigonometry_check() {
        let cnum = Complex::new();
        let zero = Complex::new();
        let one  = Complex {real: 1.0, imag: 0.0};
        assert!(one  == cnum.cosh());
        assert!(zero == cnum.sinh());
        assert!(zero == cnum.tanh());
    }

    #[test]
    fn trigonometry_check() {
        let pifourth = Complex {real: pi()/4.0, imag: 0.0};
        assert!((pifourth.cos() - pifourth.sin()).real <= 10.0f64.powi(-15));
        assert!((pifourth.tan().unwrap() - Complex {real: 1.0, imag: 0.0}).real() <= 10.0f64.powi(-15));
    }

    #[test]
    fn pi_accuracy_test() {
    	// Accuracy threshold is lower than 10^(-12)
    	assert!(pi() - 3.14159265359 < 0.0000000000001);
    }

    #[test]
    fn reduce_arg_test() {
    	let mut big = EComplex { module: 1.0, arg: 3780.0 };
    	big.reduce_arg();
    	assert!(big.arg == 180.0);
    }

    #[test]
    fn real_multiplication_check() {
    	let one   = 1.complex();
    	let e_one = 1.ecomplex();

    	let two   = 2.complex();
    	let e_two = 2.ecomplex();

    	// Assert all combinations of 1*1 are correct
    	assert!(one * one == one);
    	assert!(one * e_one == one);
    	assert!(one * e_one == e_one);
    	assert!(e_one * e_one == one);
    	assert!(e_one * e_one == e_one);

    	// Assert all combinations of 1*2 are correct
    	assert!(one * two == two);
    	assert!(e_one * two == two);
    	assert!(e_one * two == e_two);
    	assert!(one * e_two == two);
    	assert!(one * e_two == e_two);
    	assert!(e_one * e_two == e_two);
    }

    #[test]
    fn imag_multiplication_test() {
    	// This test cannot be correctly performed due to floating point not being exact
    	let i   = Complex {real: 0.0, imag: 1.0};
    	let e_i = EComplex {module: 1.0, arg: pi()/2.0f64};

    	let one   = 1.complex();
    	let e_one = 1.ecomplex();

    	// Assert combinations of 1 * i are correct
    	assert!(one * i == i);
    	assert!(one * e_i == e_i);
    	assert!(e_one * e_i == e_i);
    	assert!(e_one * i == e_i);
    }

}

/// Complex number in exponential form `(module * **_e_**^( i*argument ))`
#[derive(Debug, Clone, Copy)]
pub struct EComplex {
    pub arg: f64,
    pub module: f64,
}

/// Complex number in the usual form `(a + b*i)`
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

pub trait ComplexNumber {}

impl ComplexNumber for Complex{}
impl ComplexNumber for EComplex{}


pub trait ContainedInComplex {
    fn complex(&self)  -> Complex;
    fn ecomplex(&self) -> EComplex;
}

fn pi() -> f64 {
    (-1.0f64).acos()
}

impl EComplex {
    /// Create new EComplex equal to 0
    pub fn new() -> EComplex {
        EComplex {module: 0.0, arg: 0.0}
    }

    /// Create new EComplex equivalent to the input Complex
    pub fn new_from(f: &Complex) -> EComplex {
        EComplex {module: f.module(), arg: f.arg()}
    }

    /// Return real part of the equivalent Complex
    pub fn real(&self) -> f64 {
        self.module*self.arg.cos()
    }

    /// Return imaginary part of the equivalent Complex
    pub fn imag(&self) -> f64 {
        self.module*self.arg.sin()
    }

    /// Return the module
    pub fn module(&self) -> f64 {
        self.module
    }

    /// Return the argument in radians
    pub fn arg(&mut self) -> f64 {
        self.reduce_arg();
        self.arg
    }

    /// Return the argument in degrees
    pub fn argd(&self) -> f64 {
    	self.arg*180.0/(pi())
    }

    /// Reduce the argument to a value between [0, 2*PI]
    pub fn reduce_arg(&mut self) {
    	self.arg = self.arg - ((self.arg as i64 / 360 ) as f64) * 360.0;
    }

    /// Return a vector with the square roots
    pub fn sqrt(&self) -> Result<Vec<EComplex>, &'static str> {
        self.root(2)
    }

    /// Return a vector of `n` values with the `n`th roots
    pub fn root(&self, r: usize) -> Result<Vec<EComplex>, &'static str> {
        if r < 1 {
            return Err("Illegal value for root");
        }

        let mut out: Vec<EComplex> = vec![];
        let new_mod: f64 = self.module.powf(1.0f64/(r as f64));
        let pi: f64 = pi();

        for i in 0..r {
            out.push(EComplex {module: new_mod,
                               arg: (self.arg + 2.0*pi*(i as f64)) / (r as f64)});
        }
        Ok(out)
    }

    /// Return the number raised to the power `p`
    pub fn powi(&self, p: isize) -> EComplex {
        EComplex {module: self.module.powi(p as i32),
                  arg: self.arg*(p as f64)}
    }

    /// Return the number raised to the power `p`
    pub fn powf(&self, p: f64) -> EComplex {
        EComplex {module: self.module.powf(p),
                  arg: self.arg*p}
    }

    /// Return the conjugate EComplex
    pub fn conjugate(&self) -> EComplex {
        EComplex {module: self.module, arg: -self.arg}
    }

    /// Return module
    pub fn abs(&self) -> f64 {
        self.module
    }

    /// Return `e^(self)`
    pub fn exp(&self) -> EComplex {
        EComplex {module: self.module*self.arg.cos(), arg: self.module*self.arg.sin()}
    }

    /// Return `ln(self)`
    pub fn ln(&self) -> EComplex {
        EComplex::new_from(&Complex {real: self.module.ln(), imag: self.arg})
    }

    /// Return `log(self)` in the base `b`
    pub fn log(&self, b: f64) -> EComplex {
        self.ln() / b.ln().ecomplex()
    }

    /// Return `log(self)` in base 2
    pub fn log2(&self) -> EComplex {
    	self.ln() / 2.0f64.ln().ecomplex()
    }

    /// Return `log(self)` in base 10
    pub fn log10(&self) -> EComplex {
    	self.ln() / 10.0f64.ln().ecomplex()
    }

    /// Return `1/self`
    pub fn inverse(&self) -> EComplex {
        EComplex {module: 1.0/self.module, arg: -self.arg}
    }

    /// Return the cosine
    pub fn cos(&self) -> EComplex {
        EComplex::new_from(
            &Complex {real:  self.real().cos()*self.imag().cosh(),
                      imag: -self.real().sin()*self.imag().sinh()}
        )
    }

    /// Return the sine
    pub fn sin(&self) -> EComplex {
        EComplex::new_from(
            &Complex {real: self.real().sin()*self.imag().cosh(),
                      imag: self.real().cos()*self.imag().sinh()}
        )
    }

    /// Return the tangent
    pub fn tan(&self) -> Option<EComplex> {
        let cosine = self.cos();
        if cosine.module != 0.0 {
            Some(self.sin()/cosine)
        } else {
            None
        }
    }

    /// Return the hyperolic cosine
    pub fn cosh(&self) -> EComplex {
        EComplex::new_from(
            &Complex {real: self.real().cosh()*self.imag().cos(),
                        imag: self.real().sinh()*self.imag().sin()}
        )
    }

    /// Return the hyperbolic sine
    pub fn sinh(&self) -> EComplex {
		EComplex::new_from(
			&Complex {real: self.real().sinh()*self.imag().cos(),
						imag: self.real().cosh()*self.imag().sin()}
		)
	}

    /// Return the hyperbolic tangent
    pub fn tanh(&self) -> EComplex {
        self.sinh() / self.cosh()
    }

    /// Return the module and argument as a tuple `(mod, arg)`
	pub fn tuple(&self) -> (f64, f64) {
		(self.module, self.arg)
	}

    /// Return as a tuple the real and imaginary part of the equivalent Complex
    /// (real, imag)
	pub fn c_tuple(&self) -> (f64, f64) {
		(self.real(), self.imag())
	}
}

impl Complex {
    pub fn new() -> Complex {
        Complex {real: 0.0, imag: 0.0}
    }

    pub fn new_from(f: &EComplex) -> Complex {
        Complex {real: f.real(), imag: f.imag()}
    }

    /// Return real part
    pub fn real(&self) -> f64 {
        self.real
    }

    /// Return imaginary part
    pub fn imag(&self) -> f64 {
        self.imag
    }

    /// Return module of equivalent EComplex
    pub fn module(&self) -> f64 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }

    /// Return argument of equivalent EComplex in radians
    pub fn arg(&self) -> f64 {
        if self.module() != 0.0{
            (self.real / self.module()).acos()
        } else {
            0.0
        }
    }

    /// Return argument of equivalent EComplex in degrees
    pub fn argd(&self) -> f64 {
    	self.arg()*360.0/(2.0*pi())
    }

    /// Return vector of the two square roots
    pub fn sqrt(&self) -> Result<Vec<EComplex>, &'static str> {
        self.root(2)
    }

    /// Return vector of size `n` with the `n`th roots
    pub fn root(&self, r: usize) -> Result<Vec<EComplex>, &'static str> {
        if r < 1 {
            return Err("Illegal value for root");
        }

        let mut out: Vec<EComplex> = vec![];
        let new_mod: f64 = self.module().powf(1.0f64/(r as f64));
        let arg_k: f64 = self.arg();
        let pi = pi();

        for i in 0..r {
            out.push(EComplex {module: new_mod, arg: (arg_k + 2.0*pi*(i as f64)) / (r as f64)});
        }
        Ok(out)
    }

    /// Return the number raised to the power `p`
    pub fn powi(&self, p: isize) -> Complex {
        Complex {real: self.module().powi(p as i32)*(((p as f64)*self.arg()).cos()),
                    imag: self.module().powi(p as i32)*(((p as f64)*self.arg()).sin())}
    }
    /// Return the number raised to the power `p`
    pub fn powf(&self, p: f64) -> Complex {
        Complex {real: self.module().powf(p)*((p*self.arg()).cos()),
                    imag: self.module().powf(p)*((p*self.arg()).sin())}
    }

    /// Return the conjugate Complex
    pub fn conjugate(&self) -> Complex {
        Complex {real: self.real, imag: -self.imag}
    }

    /// Return the module
    pub fn abs(&self) -> f64 {
        self.module()
    }

    /// Return `e^(self)`
    pub fn exp(&self) -> Complex {
        Complex {real: self.imag.cos()*self.real.exp(), imag: self.imag.sin()*self.real.exp()}
    }

    /// Return `ln(self)`
    pub fn ln(&self) -> Complex{
        let othermod = (self.real*self.real + self.imag*self.imag).sqrt();
        let otherarg = (self.real / othermod).acos();
        Complex {real: othermod.ln(), imag: otherarg}
    }

    /// Return `log(self)` in the base `b`
    pub fn log(&self, b: f64) -> Complex {
        self.ln() / b.ln().complex()
    }

    /// Return `log(self)` in the base `b`
    pub fn log2(&self) -> Complex {
        self.ln() / 2.0f64.ln().complex()
    }

    /// Return `log(self)` in the base `b`
    pub fn log10(&self) -> Complex {
        self.ln() / 10.0f64.ln().complex()
    }

    /// Return `1/self`
    pub fn inverse(&self) -> Complex {
        Complex {real:  self.real / (self.real*self.real + self.imag+self.imag),
                    imag: -self.imag / (self.real*self.real + self.imag*self.imag)}
    }

    /// Return the cosine
    pub fn cos(&self) -> Complex {
        Complex {real:  self.real.cos()*self.imag.cosh(),
                 imag: -self.real.sin()*self.imag.sinh()}
    }

    /// Return the sine
    pub fn sin(&self) -> Complex {
        Complex {real: self.real.sin()*self.imag.cosh(),
                 imag: self.real.cos()*self.imag.sinh()}
    }

    /// Return the tangent
    pub fn tan(&self) -> Option<Complex> {
        let cosine = self.cos();
        if cosine.real != 0.0 || cosine.imag != 0.0 {
            Some(self.sin()/cosine)
        } else {
            None
        }
    }

    /// Return the hyperbolic cosine
    pub fn cosh(&self) -> Complex {
        Complex {real: self.real.cosh()*self.imag.cos(),
                 imag: self.real.sinh()*self.imag.sin()}
    }

    /// Return the hyperbolic sine
    pub fn sinh(&self) -> Complex {
        Complex {real: self.real.sinh()*self.imag.cos(),
                 imag: self.real.cosh()*self.imag.sin()}
    }

    /// Return the hyperbolic tangent
    pub fn tanh(&self) -> Complex {
        Complex {real: self.real.tanh(), imag: self.imag.tan()} / Complex {real: 1.0, imag: self.real.tanh() * self.imag.tan()}
    }

    /// Return the real and imaginary parts as a tuple `(real, imag)`
	pub fn tuple(&self) -> (f64, f64) {
		(self.real, self.imag)
	}

    /// Return as a tuple the module and argument of the equivalent EComplex
    /// (mod, arg)
	pub fn exp_tuple(&self) -> (f64, f64) {
		(self.module(), self.arg())
	}
}

impl ContainedInComplex for u8 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for u16 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for u32 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for u64 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for i8 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for i16 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for i32 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for i64 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for f32 {
    fn complex(&self) -> Complex {
        Complex { real: (*self as f64) , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self as f64 , arg: 0.0 }
    }
}

impl ContainedInComplex for f64 {
    fn complex(&self) -> Complex {
        Complex { real: *self , imag: 0.0 }
    }

    fn ecomplex(&self) -> EComplex {
        EComplex { module: *self , arg: 0.0 }
    }
}

impl ContainedInComplex for (u8, u8) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (u16, u16) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (u32, u32) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (u64, u64) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (i8, i8) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (i16, i16) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (i32, i32) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (i64, i64) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (f32, f32) {

    fn complex(&self) -> Complex {
        Complex { real: self.0 as f64, imag: self.1 as f64 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0 as f64 , arg: self.1 as f64 }
    }
}

impl ContainedInComplex for (f64, f64) {

    fn complex(&self) -> Complex {
        Complex { real: self.0, imag: self.1 }
    }

    fn ecomplex(&self) -> EComplex {
    	EComplex { module: self.0, arg: self.1 }
    }
}

impl From<EComplex> for Complex {
    fn from(num: EComplex) -> Self {
        Complex {real: num.real(), imag: num.imag()}
    }
}

impl From<Complex> for EComplex {
    fn from(num: Complex) -> Self {
        EComplex {module: num.module(), arg: num.arg()}
    }
}

#[cfg(feature="num_complex_compatible")]
impl From<Complex64> for EComplex {
    fn from(num: Complex64) -> Self {
        EComplex::new_from(&Complex {real: num.re, imag: num.im})
    }
}

#[cfg(feature = "num_complex_compatible")]
impl From<Complex64> for Complex {
    fn from(num: Complex64) -> Self {
        Complex {real: num.re, imag: num.im}
    }
}

#[cfg(feature="num_complex_compatible")]
impl From<EComplex> for Complex64 {
    fn from(num: EComplex) -> Self {
        Complex64::new(num.real(), num.imag())
    }
}

#[cfg(feature="num_complex_compatible")]
impl From<Complex> for Complex64 {
    fn from(num: Complex) -> Self {
        Complex64::new(num.real, num.imag)
    }
}

impl fmt::Display for EComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} · exp({}j)", self.module, self.arg)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}j", self.real, if self.imag >= 0.0 {"+"} else {"-"}, self.imag)
    }
}

impl PartialEq<Complex> for EComplex {
    fn eq(&self, other: &Complex) -> bool {
        if self.real() == other.real() {
            self.imag() == other.imag()
        } else {
            false
        }
    }

    fn ne(&self, other: &Complex) -> bool {
        if self.real() != other.real() {
            true
        } else {
            self.imag() != other.imag()
        }
    }
}

impl PartialEq<EComplex> for Complex {
    fn eq(&self, other: &EComplex) -> bool {
        if self.real() == other.real() {
            self.imag() == other.imag()
        } else {
            false
        }
    }

    fn ne(&self, other: &EComplex) -> bool {
        if self.real() != other.real() {
            true
        } else {
            self.imag() != other.imag()
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        if self.real == other.real {
            self.imag == other.imag
        } else {
            false
        }
    }

    fn ne(&self, other: &Complex) -> bool {
        if self.real != other.real {
            true
        } else {
            self.imag != other.imag
        }
    }
}

impl PartialEq for EComplex {
    fn eq(&self, other: &EComplex) -> bool {
        if self.real() == other.real() {
            self.imag() == other.imag()
        } else {
            false
        }
    }

    fn ne(&self, other: &EComplex) -> bool {
        if self.real() != other.real() {
            true
        } else {
            self.imag() != other.imag()
        }
    } 
}

impl Add<EComplex> for EComplex {
    type Output = EComplex;

    fn add(self, other: EComplex) -> EComplex {
        let newmodule = ((self.real() + other.real()).powi(2) + (self.imag() + other.imag()).powi(2)).sqrt();
        let newarg = if newmodule != 0.0 {
            ((self.real() + other.real())/newmodule).acos()
        } else {
            0.0
        };

       EComplex {module: newmodule, arg: newarg}
    }
}

impl<'a, 'b> Add<&'b EComplex> for &'a EComplex {
    type Output = EComplex;

    fn add(self, other: &'b EComplex) -> EComplex {
        let newmodule = ((self.real() + other.real()).powi(2) + (self.imag() + other.imag()).powi(2)).sqrt();
        let newarg = if newmodule != 0.0 {
            ((self.real() + other.real())/newmodule).acos()
        } else {
            0.0
        };

       EComplex {module: newmodule, arg: newarg}
    }
}

impl Add<Complex> for EComplex {
    type Output = EComplex;

    fn add(self, other: Complex) -> EComplex {
        let newreal = self.real() + other.real;
        let newimag = self.imag() + other.imag;

        let newmod = (newreal*newreal + newimag*newimag).sqrt();
        let newarg = (newreal/newimag).atan();

        EComplex {module: newmod, arg: newarg}
    }
}

impl<'a, 'b> Add<&'b Complex> for &'a EComplex {
    type Output = EComplex;

    fn add(self, other: &'b Complex) -> EComplex {
        let newreal = self.real() + other.real;
        let newimag = self.imag() + other.imag;

        let newmod = (newreal*newreal + newimag*newimag).sqrt();
        let newarg = (newreal/newimag).atan();

        EComplex {module: newmod, arg: newarg}
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl<'a, 'b> Add<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn add(self, other: &'b Complex) -> Complex {
        Complex {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl Add<EComplex> for Complex {
    type Output = Complex;

    fn add(self, other: EComplex) -> Complex {
        Complex {real: self.real + other.real(), imag: self.imag + other.imag()}
    }
}

impl<'a, 'b> Add<&'b EComplex> for &'a Complex {
    type Output = Complex;

    fn add(self, other: &'b EComplex) -> Complex {
        Complex {real: self.real + other.real(), imag: self.imag + other.imag()}
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {real: -self.real, imag: -self.imag}
    }
}

impl<'a> Neg for &'a Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {real: -self.real, imag: -self.imag}
    }
}

impl Neg for EComplex {
    type Output = EComplex;

    // If arg > 0 then spin one radian counterclockwise, else if arg < 0 spin clockwise
    fn neg(self) -> EComplex {
        EComplex {module: self.module, arg: if self.arg > 0.0 {self.arg - pi()} else {self.arg + pi()}}
    }
}

impl<'a> Neg for &'a EComplex {
    type Output = EComplex;

    // If arg > 0 then spin one radian counterclockwise, else if arg < 0 spin clockwise
    fn neg(self) -> EComplex {
        EComplex {module: self.module, arg: if self.arg > 0.0 {self.arg - pi()} else {self.arg + pi()}}
    }
}

impl Sub<EComplex> for EComplex {
    type Output = EComplex;

    fn sub(self, other: EComplex) -> EComplex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b EComplex> for &'a EComplex {
    type Output = EComplex;

    fn sub(self, other: &'b EComplex) -> EComplex {
        self + &(-other)
    }
}

impl Sub<Complex> for EComplex {
    type Output = EComplex;

    fn sub(self, other: Complex) -> EComplex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b Complex> for &'a EComplex {
    type Output = EComplex;

    fn sub(self, other: &'b Complex) -> EComplex {
        self + &(-other)
    }
}

impl Sub<EComplex> for Complex {
    type Output = Complex;

    fn sub(self, other: EComplex) -> Complex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b EComplex> for &'a Complex {
    type Output = Complex;

    fn sub(self, other: &'b EComplex) -> Complex {
        self + &(-other)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn sub(self, other: &'b Complex) -> Complex {
        self + &(-other)
    }
}

impl Mul<EComplex> for EComplex {
    type Output = EComplex;

    fn mul(self, other: EComplex) -> EComplex {
        EComplex {module: self.module * other.module, arg: self.arg + other.arg}
    }
}

impl<'a, 'b> Mul<&'b EComplex> for &'a EComplex {
    type Output = EComplex;

    fn mul(self, other: &'b EComplex) -> EComplex {
        EComplex {module: self.module * other.module, arg: self.arg + other.arg}
    }
}

impl Mul<Complex> for EComplex {
    type Output = EComplex;

    fn mul(self, other: Complex) -> EComplex {
        EComplex {module: self.module * other.module(), arg: self.arg + other.arg()}
    }
}

impl<'a, 'b> Mul<&'b Complex> for &'a EComplex {
    type Output = EComplex;

    fn mul(self, other: &'b Complex) -> EComplex {
        EComplex {module: self.module * other.module(), arg: self.arg + other.arg()}
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {real: self.real*other.real - self.imag*other.imag,
                 imag: self.real*other.imag + self.imag*other.real}
    }
}

impl<'a, 'b> Mul<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn mul(self, other: &'b Complex) -> Complex {
        Complex {real: self.real*other.real - self.imag*other.imag,
                 imag: self.real*other.imag + self.imag*other.real}
    }
}

impl Mul<EComplex> for Complex {
    type Output = Complex;

    fn mul(self, other: EComplex) -> Complex {
        Complex {real: self.real*other.real() - self.imag*other.imag(),
                 imag: self.real*other.imag() + self.imag*other.real()}
    }
}

impl<'a, 'b> Mul<&'b EComplex> for &'a Complex {
    type Output = Complex;

    fn mul(self, other: &'b EComplex) -> Complex {
        Complex {real: self.real*other.real() - self.imag*other.imag(),
                 imag: self.real*other.imag() + self.imag*other.real()}
    }
}

impl Div<EComplex> for EComplex {
    type Output = EComplex;

    fn div(self, other: EComplex) -> EComplex {
        EComplex {module: self.module/other.module, arg: self.arg - other.arg}
    }
}

impl<'a, 'b> Div<&'b EComplex> for &'a EComplex {
    type Output = EComplex;

    fn div(self, other: &'b EComplex) -> EComplex {
        EComplex {module: self.module/other.module, arg: self.arg - other.arg}
    }
}

impl Div<Complex> for EComplex {
    type Output = EComplex;

    fn div(self, other: Complex) -> EComplex {
        EComplex {module: self.module/other.module(), arg: self.arg - other.arg()}
    }
}

impl<'a, 'b> Div<&'b Complex> for &'a EComplex {
    type Output = EComplex;

    fn div(self, other: &'b Complex) -> EComplex {
        EComplex {module: self.module/other.module(), arg: self.arg - other.arg()}
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex {real: (self.real*other.real + self.imag*other.imag) / (other.real*other.real + other.imag*other.imag),
                 imag: (self.imag*other.imag - self.real*other.imag) / (other.real*other.real + other.imag*other.imag)}
    }
}

impl<'a, 'b> Div<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn div(self, other: &'b Complex) -> Complex {
        Complex {real: (self.real*other.real + self.imag*other.imag) / (other.real*other.real + other.imag*other.imag),
                 imag: (self.imag*other.imag - self.real*other.imag) / (other.real*other.real + other.imag*other.imag)}
    }
}

impl Div<EComplex> for Complex {
    type Output = Complex;

    fn div(self, other: EComplex) -> Complex {
        Complex {real: (self.real*other.real() + self.imag*other.imag()) / (other.real()*other.real() + other.imag()*other.imag()),
                 imag: (self.imag*other.imag() - self.real*other.imag()) / (other.real()*other.real() + other.imag()*other.imag())}
    }
}

impl<'a, 'b> Div<&'b EComplex> for &'a Complex {
    type Output = Complex;

    fn div(self, other: &'b EComplex) -> Complex {
        Complex {real: (self.real*other.real() + self.imag*other.imag()) / (other.real()*other.real() + other.imag()*other.imag()),
                 imag: (self.imag*other.imag() - self.real*other.imag()) / (other.real()*other.real() + other.imag()*other.imag())}
    }
}
