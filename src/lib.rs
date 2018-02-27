//!easy-complex crate
//!Crate for easy to use complex numbers
//!Only use of std library

use std::ops::{Add, Mul, Neg, Sub, Div};
use std::fmt;


#[cfg(feature="nightly")]
use std::num::{Zero, One};

#[cfg(feature="num_complex_compatible")]
extern crate num_complex;

#[cfg(feature="num_complex_compatible")]
use num_complex::Complex64;

#[derive(Clone, Copy)]
pub struct ExpComplex {
    pub arg: f64,
    pub module: f64,
}

#[derive(Clone, Copy)]
pub struct NumComplex {
    pub real: f64,
    pub imag: f64,
}

pub trait Complex {}

impl Complex for NumComplex{}
impl Complex for ExpComplex{}


pub trait ContainedInComplex {
    fn complex(&self) -> ExpComplex;
}

fn pi() -> f64 {
    (-1.0f64).acos()
}

impl ExpComplex {
    pub fn new() -> ExpComplex {
        ExpComplex {module: 0.0, arg: 0.0}
    }

    pub fn new_from(f: &NumComplex) -> ExpComplex {
        ExpComplex {module: f.module(), arg: f.arg()}
    }

    pub fn real(&self) -> f64 {
        self.module*self.arg.cos()
    }

    pub fn imag(&self) -> f64 {
        self.module*self.arg.sin()
    }

    pub fn d_arg(&self) -> f64 {
    	self.arg*360.0/(2.0*pi())
    }

    pub fn reduce_arg(&mut self) {
    	self.arg = self.arg.cos().acos();
    }

    pub fn sqrt(&self) -> Result<Vec<ExpComplex>, &'static str> {
        self.root(2)
    }

    pub fn root(&self, r: usize) -> Result<Vec<ExpComplex>, &'static str> {
        if r < 1 {
            return Err("Illegal value for root");
        }

        let mut out: Vec<ExpComplex> = vec![];
        let new_mod: f64 = self.module.powf(1.0f64/(r as f64));
        let pi: f64 = pi();

        for i in 0..r {
            out.push(ExpComplex {module: new_mod,
                                arg: (self.arg + 2.0*pi*(i as f64)) / (r as f64)});
        }
        Ok(out)
    }

    pub fn powi(&self, p: isize) -> ExpComplex {
        ExpComplex {module: self.module.powi(p as i32),
                    arg: self.arg*(p as f64)}
    }

    pub fn powf(&self, p: f64) -> ExpComplex {
        ExpComplex {module: self.module.powf(p),
                    arg: self.arg*p}
    }

    pub fn conjugate(&self) -> ExpComplex {
        ExpComplex {module: self.module, arg: -self.arg}
    }

    pub fn abs(&self) -> f64 {
        self.module
    }

    pub fn exp(&self) -> ExpComplex {
        ExpComplex {module: self.module*self.arg.cos(), arg: self.module*self.arg.sin()}
    }

    pub fn ln(&self) -> ExpComplex {
        ExpComplex::new_from(&NumComplex {real: self.module.ln(), imag: self.arg})
    }

    pub fn log(&self, base: f64) -> ExpComplex {
        self.ln() / base.ln().complex()
    }

    pub fn inverse(&self) -> ExpComplex {
        ExpComplex {module: 1.0/self.module, arg: -self.arg}
    }

    pub fn cos(&self) -> ExpComplex {
        ExpComplex::new_from(
            &NumComplex {real:  self.real().cos()*self.imag().cosh(),
                        imag: -self.real().sin()*self.imag().sinh()}
        )
    }

    pub fn sin(&self) -> ExpComplex {
        ExpComplex::new_from(
            &NumComplex {real: self.real().sin()*self.imag().cosh(),
                        imag: self.real().cos()*self.imag().sinh()}
        )
    }

    pub fn tan(&self) -> Option<ExpComplex> {
        let cosine = self.cos();
        if cosine.module != 0.0 {
            Some(self.sin()/cosine)
        } else {
            None
        }
    }

    pub fn cosh(&self) -> ExpComplex {
        ExpComplex::new_from(
            &NumComplex {real: self.real().cosh()*self.imag().cos(),
                        imag: self.real().sinh()*self.imag().sin()}
        )
    }

    pub fn sinh(&self) -> ExpComplex {
		ExpComplex::new_from(
			&NumComplex {real: self.real().sinh()*self.imag().cos(),
						imag: self.real().cosh()*self.imag().sin()}
		)
	}

	pub fn tuple(&self) -> (f64, f64) {
		(self.module, self.arg)
	}

	pub fn coord_tuple(&self) -> (f64, f64) {
		(self.real(), self.imag())
	}
}

impl NumComplex {
    pub fn new() -> NumComplex {
        NumComplex {real: 0.0, imag: 0.0}
    }

    pub fn new_from(f: &ExpComplex) -> NumComplex {
        NumComplex {real: f.real(), imag: f.imag()}
    }

    pub fn module(&self) -> f64 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }

    pub fn arg(&self) -> f64 {
        if self.module() != 0.0{
            (self.real / self.module()).acos()
        } else {
            0.0
        }
    }

    pub fn d_arg(&self) -> f64 {
    	self.arg()*360.0/(2.0*pi())
    }

    pub fn sqrt(&self) -> Result<Vec<ExpComplex>, &'static str> {
        self.root(2)
    }

    pub fn root(&self, r: usize) -> Result<Vec<ExpComplex>, &'static str> {
        if r < 1 {
            return Err("Illegal value for root");
        }

        let mut out: Vec<ExpComplex> = vec![];
        let new_mod: f64 = self.module().powf(1.0f64/(r as f64));
        let arg_k: f64 = self.arg();
        let pi = pi();

        for i in 0..r {
            out.push(ExpComplex {module: new_mod, arg: (arg_k + 2.0*pi*(i as f64)) / (r as f64)});
        }
        Ok(out)
    }

    pub fn powi(&self, p: isize) -> NumComplex {
        NumComplex {real: self.module().powi(p as i32)*(((p as f64)*self.arg()).cos()),
                    imag: self.module().powi(p as i32)*(((p as f64)*self.arg()).sin())}
    }

    pub fn powf(&self, p: f64) -> NumComplex {
        NumComplex {real: self.module().powf(p)*((p*self.arg()).cos()),
                    imag: self.module().powf(p)*((p*self.arg()).sin())}
    }

    pub fn conjugate(&self) -> NumComplex {
        NumComplex {real: self.real, imag: -self.imag}
    }

    pub fn abs(&self) -> f64 {
        self.module()
    }

    pub fn exp(&self) -> NumComplex {
        NumComplex {real: self.imag.cos()*self.real.exp(), imag: self.imag.sin()*self.real.exp()}
    }

    pub fn ln(&self) -> NumComplex{
        let othermod = (self.real*self.real + self.imag*self.imag).sqrt();
        let otherarg = (self.real / othermod).acos();
        NumComplex {real: othermod.ln(), imag: otherarg}
    }

    pub fn log(&self, base: f64) -> NumComplex {
        self.ln() / base.ln().complex()
    }

    pub fn inverse(&self) -> NumComplex {
        NumComplex {real:  self.real / (self.real*self.real + self.imag+self.imag),
                    imag: -self.imag / (self.real*self.real + self.imag*self.imag)}
    }

    pub fn cos(&self) -> NumComplex {
        NumComplex {real:  self.real.cos()*self.imag.cosh(),
                    imag: -self.real.sin()*self.imag.sinh()}
    }

    pub fn sin(&self) -> NumComplex {
        NumComplex {real: self.real.sin()*self.imag.cosh(),
                    imag: self.real.cos()*self.imag.sinh()}
    }

    pub fn tan(&self) -> Option<NumComplex> {
        let cosine = self.cos();
        if cosine.real != 0.0 || cosine.imag != 0.0 {
            Some(self.sin()/cosine)
        } else {
            None
        }
    }

    pub fn cosh(&self) -> NumComplex {
        NumComplex {real: self.real.cosh()*self.imag.cos(),
                    imag: self.real.sinh()*self.imag.sin()}
    }

    pub fn sinh(&self) -> NumComplex {
        NumComplex {real: self.real.sinh()*self.imag.cos(),
                    imag: self.real.cosh()*self.imag.sin()}
    }

	pub fn tuple(&self) -> (f64, f64) {
		(self.real, self.imag)
	}

	pub fn exp_tuple(&self) -> (f64, f64) {
		(self.module(), self.arg())
	}
}

impl ContainedInComplex for u8 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for u16 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for u32 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for u64 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for i8 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for i16 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for i32 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for i64 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for f32 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for f64 {
    fn complex(&self) -> ExpComplex {
        ExpComplex {module: (*self as f64), arg: 0.0}
    }
}

impl ContainedInComplex for (u8, u8) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (u16, u16) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (u32, u32) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (u64, u64) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (i8, i8) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (i16, i16) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (i32, i32) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (i64, i64) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (f32, f32) {
    fn complex(&self) -> ExpComplex {
        let newmodule = ((self.0*self.0 + self.1*self.1) as f64).sqrt();
        let newarg = if self.0 != 0.0 {
            ((self.0 as f64)/newmodule).acos()
        } else if self.1 != 0.0 {
            ((self.1 as f64)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl ContainedInComplex for (f64, f64) {
    fn complex(&self) -> ExpComplex {
        let newmodule = (self.0*self.0 + self.1*self.1).sqrt();
        let newarg = if self.0 != 0.0 {
            ((self.0)/newmodule).acos()
        } else if self.1 != 0.0 {
            ((self.1)/newmodule).asin()
        } else {
            0.0f64
        };
        ExpComplex {module: newmodule, arg: newarg}
    }
}

impl From<ExpComplex> for NumComplex {
    fn from(num: ExpComplex) -> Self {
        NumComplex {real: num.real(), imag: num.imag()}
    }
}

impl From<NumComplex> for ExpComplex {
    fn from(num: NumComplex) -> Self {
        ExpComplex {module: num.module(), arg: num.arg()}
    }
}

#[cfg(feature="num_complex_compatible")]
impl From<Complex64> for ExpComplex {
    fn from(num: Complex64) -> Self {
        ExpComplex::new_from(&NumComplex {real: num.re, imag: num.im})
    }
}

#[cfg(feature = "num_complex_compatible")]
impl From<Complex64> for NumComplex {
    fn from(num: Complex64) -> Self {
        NumComplex {real: num.re, imag: num.im}
    }
}

#[cfg(feature="num_complex_compatible")]
impl From<ExpComplex> for Complex64 {
    fn from(num: ExpComplex) -> Self {
        Complex64::new(num.real(), num.imag())
    }
}

#[cfg(feature="num_complex_compatible")]
impl From<NumComplex> for Complex64 {
    fn from(num: NumComplex) -> Self {
        Complex64::new(num.real, num.imag)
    }
}

impl fmt::Display for ExpComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} · exp({}j)", self.module, self.arg)
    }
}

impl fmt::Display for NumComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}j", self.real, if self.imag >= 0.0 {"+"} else {"-"}, self.imag)
    }
}
pub fn euler_id() {
    let pi = (-1.0f64).acos();
    let exp_number = ExpComplex {module: 1.0, arg: pi};
    println!("Euler's identity: {} + 1 = {}", exp_number, exp_number + 1.complex());
}

impl PartialEq for ExpComplex {
    fn eq(&self, other: &ExpComplex) -> bool {
        if self.module == other.module {
            return self.arg.cos() == other.arg.cos() && self.arg.sin() == other.arg.sin();
        }
        false
    }

    fn ne(&self, other: &ExpComplex) -> bool {
        if self.module != other.module || self.arg.cos() != other.arg.cos() || self.arg.sin() != other.arg.sin() {
            true
        } else {
            false
        }
    }
}

impl PartialEq for NumComplex {
    fn eq(&self, other: &NumComplex) -> bool {
        return self.real == other.real && self.imag == other.imag;
    }

    fn ne(&self, other: &NumComplex) -> bool {
        return !(self.real == other.real && self.imag == other.imag);
    }
}

#[cfg(feature="nightly")]
impl Zero for ExpComplex {
    #[inline]
    fn zero() -> ExpComplex {
        ExpComplex {module: 0.0f64, arg: 0.0f64}
    }

    fn is_zero(&self) -> bool {
        self.module == 0.0
    }
}

#[cfg(feature="nightly")]
impl One for ExpComplex {
    #[inline]
    fn one() -> ExpComplex {
        ExpComplex {module: 1.0f64, arg: 0.0f64}
    }

    fn is_zero(&self) -> bool {
        return self.real == 0.0 && self.imag == 0.0
    }
}

#[cfg(feature="nightly")]
impl Zero for NumComplex {
    #[inline]
    fn zero() -> NumComplex {
        NumComplex {real: 0.0f64, imag: 0.0f64}
    }
}

#[cfg(feature="nightly")]
impl One for NumComplex {
    #[inline]
    fn one() -> NumComplex {
        NumComplex {real: 1.0f64, imag: 0.0f64}
    }
}

impl Add<ExpComplex> for ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: ExpComplex) -> ExpComplex {
        let newmodule = ((self.real() + other.real()).powi(2) + (self.imag() + other.imag()).powi(2)).sqrt();
        let newarg = if newmodule != 0.0 {
            ((self.real() + other.real())/newmodule).acos()
        } else {
            0.0
        };

       ExpComplex {module: newmodule, arg: newarg}
    }
}

impl<'a, 'b> Add<&'b ExpComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: &'b ExpComplex) -> ExpComplex {
        let newmodule = ((self.real() + other.real()).powi(2) + (self.imag() + other.imag()).powi(2)).sqrt();
        let newarg = if newmodule != 0.0 {
            ((self.real() + other.real())/newmodule).acos()
        } else {
            0.0
        };

       ExpComplex {module: newmodule, arg: newarg}
    }
}

impl Add<NumComplex> for ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: NumComplex) -> ExpComplex {
        let newreal = self.real() + other.real;
        let newimag = self.imag() + other.imag;

        let newmod = (newreal*newreal + newimag*newimag).sqrt();
        let newarg = if newmod != 0.0 {
                (newreal/newmod).acos()
        } else {
                0.0
        };
        ExpComplex {module: newmod, arg: newarg}
    }
}

impl<'a, 'b> Add<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: &'b NumComplex) -> ExpComplex {
        let newreal = self.real() + other.real;
        let newimag = self.imag() + other.imag;

        let newmod = (newreal*newreal + newimag*newimag).sqrt();
        let newarg = if newmod != 0.0 {
                (newreal/newmod).acos()
        } else {
                0.0
        };
        ExpComplex {module: newmod, arg: newarg}
    }
}

impl Add<NumComplex> for NumComplex {
    type Output = NumComplex;

    fn add(self, other: NumComplex) -> NumComplex {
        NumComplex {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl<'a, 'b> Add<&'b NumComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn add(self, other: &'b NumComplex) -> NumComplex {
        NumComplex {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl Add<ExpComplex> for NumComplex {
    type Output = NumComplex;

    fn add(self, other: ExpComplex) -> NumComplex {
        NumComplex {real: self.real + other.module*other.arg.cos(), imag: self.imag + other.module*other.arg.sin()}
    }
}

impl<'a, 'b> Add<&'b ExpComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn add(self, other: &'b ExpComplex) -> NumComplex {
        NumComplex {real: self.real + other.module*other.arg.cos(), imag: self.imag + other.module*other.arg.sin()}
    }
}

impl Neg for NumComplex {
    type Output = NumComplex;

    fn neg(self) -> NumComplex {
        NumComplex {real: -self.real, imag: -self.imag}
    }
}

impl<'a> Neg for &'a NumComplex {
    type Output = NumComplex;

    fn neg(self) -> NumComplex {
        NumComplex {real: -self.real, imag: -self.imag}
    }
}

impl Neg for ExpComplex {
    type Output = ExpComplex;

    fn neg(self) -> ExpComplex {
        ExpComplex {module: self.module, arg: if self.arg > 0.0 {self.arg - pi()} else {self.arg + pi()}}
    }
}

impl<'a> Neg for &'a ExpComplex {
    type Output = ExpComplex;

    fn neg(self) -> ExpComplex {
        ExpComplex {module: self.module, arg: if self.arg > 0.0 {self.arg - pi()} else if self.arg < 0.0 {self.arg + pi()} else { pi()/2.0}}
    }
}

impl Sub<ExpComplex> for ExpComplex {
    type Output = ExpComplex;

    fn sub(self, other: ExpComplex) -> ExpComplex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b ExpComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn sub(self, other: &'b ExpComplex) -> ExpComplex {
        self + &(-other)
    }
}

impl Sub<NumComplex> for ExpComplex {
    type Output = ExpComplex;

    fn sub(self, other: NumComplex) -> ExpComplex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn sub(self, other: &'b NumComplex) -> ExpComplex {
        self + &(-other)
    }
}

impl Sub<ExpComplex> for NumComplex {
    type Output = NumComplex;

    fn sub(self, other: ExpComplex) -> NumComplex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b ExpComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn sub(self, other: &'b ExpComplex) -> NumComplex {
        self + &(-other)
    }
}

impl Sub<NumComplex> for NumComplex {
    type Output = NumComplex;

    fn sub(self, other: NumComplex) -> NumComplex {
        self + (-other)
    }
}

impl<'a, 'b> Sub<&'b NumComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn sub(self, other: &'b NumComplex) -> NumComplex {
        self + &(-other)
    }
}

impl Mul<ExpComplex> for ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: ExpComplex) -> ExpComplex {
        ExpComplex {module: self.module * other.module, arg: self.arg + other.arg}
    }
}

impl<'a, 'b> Mul<&'b ExpComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: &'b ExpComplex) -> ExpComplex {
        ExpComplex {module: self.module * other.module, arg: self.arg + other.arg}
    }
}

impl Mul<NumComplex> for ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: NumComplex) -> ExpComplex {
        let othermod = other.module();
        let otherarg = other.module();
        ExpComplex {module: self.module * othermod, arg: self.arg + otherarg}
    }
}

impl<'a, 'b> Mul<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: &'b NumComplex) -> ExpComplex {
        let othermod = other.module();
        let otherarg = other.module();
        ExpComplex {module: self.module * othermod, arg: self.arg + otherarg}
    }
}

impl Mul<NumComplex> for NumComplex {
    type Output = NumComplex;

    fn mul(self, other: NumComplex) -> NumComplex {
        NumComplex {real: self.real*other.real - self.imag*other.imag,
                    imag: self.real*other.imag + self.imag*other.real}
    }
}

impl<'a, 'b> Mul<&'b NumComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn mul(self, other: &'b NumComplex) -> NumComplex {
        NumComplex {real: self.real*other.real - self.imag*other.imag,
                    imag: self.real*other.imag + self.imag*other.real}
    }
}

impl Mul<ExpComplex> for NumComplex {
    type Output = NumComplex;

    fn mul(self, other: ExpComplex) -> NumComplex {
        NumComplex {real: self.real*other.real() - self.imag*other.imag(),
                    imag: self.real*other.imag() + self.imag*other.real()}
    }
}

impl<'a, 'b> Mul<&'b ExpComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn mul(self, other: &'b ExpComplex) -> NumComplex {
        NumComplex {real: self.real*other.real() - self.imag*other.imag(),
                    imag: self.real*other.imag() + self.imag*other.real()}
    }
}

impl Div<ExpComplex> for ExpComplex {
    type Output = ExpComplex;

    fn div(self, other: ExpComplex) -> ExpComplex {
        ExpComplex {module: self.module/other.module, arg: self.arg - other.arg}
    }
}

impl<'a, 'b> Div<&'b ExpComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn div(self, other: &'b ExpComplex) -> ExpComplex {
        ExpComplex {module: self.module/other.module, arg: self.arg - other.arg}
    }
}

impl Div<NumComplex> for ExpComplex {
    type Output = ExpComplex;

    fn div(self, other: NumComplex) -> ExpComplex {
        let othermod = other.module();
        let otherarg = other.arg();
        ExpComplex {module: self.module/othermod, arg: self.arg - otherarg}
    }
}

impl<'a, 'b> Div<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn div(self, other: &'b NumComplex) -> ExpComplex {
        let othermod = other.module();
        let otherarg = other.arg();
        ExpComplex {module: self.module/othermod, arg: self.arg - otherarg}
    }
}

impl Div<NumComplex> for NumComplex {
    type Output = NumComplex;

    fn div(self, other: NumComplex) -> NumComplex {
        NumComplex {real: (self.real*other.real + self.imag*other.imag) / (other.real*other.real + other.imag*other.imag),
                    imag: (self.imag*other.imag - self.real*other.imag) / (other.real*other.real + other.imag*other.imag)}
    }
}

impl<'a, 'b> Div<&'b NumComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn div(self, other: &'b NumComplex) -> NumComplex {
        NumComplex {real: (self.real*other.real + self.imag*other.imag) / (other.real*other.real + other.imag*other.imag),
                    imag: (self.imag*other.imag - self.real*other.imag) / (other.real*other.real + other.imag*other.imag)}
    }
}

impl Div<ExpComplex> for NumComplex {
    type Output = NumComplex;

    fn div(self, other: ExpComplex) -> NumComplex {
        NumComplex {real: (self.real*other.real() + self.imag*other.imag()) / (other.real()*other.real() + other.imag()*other.imag()),
                    imag: (self.imag*other.imag() - self.real*other.imag()) / (other.real()*other.real() + other.imag()*other.imag())}
    }
}

impl<'a, 'b> Div<&'b ExpComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn div(self, other: &'b ExpComplex) -> NumComplex {
        NumComplex {real: (self.real*other.real() + self.imag*other.imag()) / (other.real()*other.real() + other.imag()*other.imag()),
                    imag: (self.imag*other.imag() - self.real*other.imag()) / (other.real()*other.real() + other.imag()*other.imag())}
    }
}
