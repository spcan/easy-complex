//!easy-complex crate
//!Crate for easy to use complex numbers
//!Only use of std library

use std::ops::{Add, Mul, Neg, Sub, Div};
use std::fmt;

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

///Explicit casting for all std numeric types
///Returns a ExpComplex
///
///#Examples
///```
/// extern crate easy_complex;
/// use easy_complex::ContainedInComplex;
///
/// let _complex = 1.complex();
///
/// let i = 2;
/// _complex = i.complex();
///```
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

    pub fn new_from(f: NumComplex) -> ExpComplex {
        ExpComplex {module: f.module(), arg: f.arg()}
    }

    pub fn real(&self) -> f64 {
        self.module*self.arg.cos()
    }

    pub fn imag(&self) -> f64 {
        self.module*self.arg.sin()
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

    pub fn pow(&self, p: usize) -> ExpComplex {
        ExpComplex {module: self.module.powi(p as i32),
                    arg: self.arg*(p as f64)}
    }

    pub fn conjugate(&self) -> ExpComplex {
        ExpComplex {module: self.module, arg: -self.arg}
    }

    pub fn abs(&self) -> f64 {
        self.module
    }

    pub fn ln(&self) -> ExpComplex {
        ExpComplex::new_from(NumComplex {real: self.module.ln(), imag: self.arg})
    }
}

impl NumComplex {
    pub fn new() -> NumComplex {
        NumComplex {real: 0.0, imag: 0.0}
    }

    pub fn new_from(f: ExpComplex) -> NumComplex {
        NumComplex {real: f.real(), imag: f.imag()}
    }

    pub fn module(&self) -> f64 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }

    pub fn arg(&self) -> f64 {
        if self.real != 0.0{
            (self.real / self.module()).acos()
        } else if self.imag != 0.0{
            (self.imag / self.module()).asin()
        } else {
            0.0
        }
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

    pub fn pow(&self, p: usize) -> NumComplex {
        NumComplex {real: self.module().powi(p as i32)*(((p as f64)*self.arg()).cos()),
                    imag: self.module().powi(p as i32)*(((p as f64)*self.arg()).sin())}
    }

    pub fn conjugate(&self) -> NumComplex {
        NumComplex {real: self.real, imag: -self.imag}
    }

    pub fn abs(&self) -> f64 {
        self.module()
    }

    pub fn ln(&self) -> NumComplex{
        let othermod = (self.real*self.real + self.imag*self.imag).sqrt();
        let otherarg = (self.real / othermod).acos();
        NumComplex {real: othermod.ln(), imag: otherarg}
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

///Display of the numbers with a general computation style
///
///#Examples
///```
/// extern crate easy_complex;
/// use easy_complex::{ExpComplex, NumComplex, ContainedInComplex};
///
/// let exp = 1.complex();
/// println!("{}", exp);
/// //Prints in exponential form --> 1.0 · exp(0.0j)
/// println!("{}", NumComplex::new_from(exp));
/// //Prints in coordinates --> 1.0 + 0.0j
///```
impl fmt::Display for ExpComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} · exp({}j)", self.module, self.arg)
    }
}

impl fmt::Display for NumComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}j", self.real, if self.imag > 0.0 {"+"} else {""}, self.imag)
    }
}

impl Add<ExpComplex> for ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: ExpComplex) -> ExpComplex {
        ExpComplex {module: (self.module*self.module + other.module*other.module + 2.0*self.module*other.module*(self.arg-other.arg).cos()).sqrt(),
                    arg: ((self.module*self.arg.sin() + other.module*other.arg.sin()) / (self.module*self.arg.cos() + other.module*other.arg.cos())).atan()}
    }
}

impl<'a, 'b> Add<&'b ExpComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: &'b ExpComplex) -> ExpComplex {
        ExpComplex {module: (self.module*self.module + other.module*other.module + 2.0*self.module*other.module*(self.arg-other.arg).cos()).sqrt(),
                    arg: ((self.module*self.arg.sin() + other.module*other.arg.sin()) / (self.module*self.arg.cos() + other.module*other.arg.cos())).atan()}
    }
}

impl Add<NumComplex> for ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: NumComplex) -> ExpComplex {
        let othermod = (other.real*other.real + other.imag*other.imag).sqrt();
        let otherarg = (other.real / othermod).acos();
        ExpComplex {module: (self.module*self.module + othermod*othermod + 2.0*self.module*othermod*(self.arg-otherarg).cos()).sqrt(),
                    arg: ((self.module*self.arg.sin() + othermod*otherarg.sin()) / (self.module*self.arg.cos() + othermod*otherarg.cos())).atan()}
    }
}

impl<'a, 'b> Add<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn add(self, other: &'b NumComplex) -> ExpComplex {
        let othermod = (other.real*other.real + other.imag*other.imag).sqrt();
        let otherarg = (other.real / othermod).acos();
        ExpComplex {module: (self.module*self.module + othermod*othermod + 2.0*self.module*othermod*(self.arg-otherarg).cos()).sqrt(),
                    arg: ((self.module*self.arg.sin() + othermod*otherarg.sin()) / (self.module*self.arg.cos() + othermod*otherarg.cos())).atan()}
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
        ExpComplex {module: self.module, arg: if self.arg > 0.0 {self.arg - pi()} else {self.arg + pi()}}
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
        ExpComplex {module: self.module + other.module, arg: self.arg + other.arg}
    }
}

impl<'a, 'b> Mul<&'b ExpComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: &'b ExpComplex) -> ExpComplex {
        ExpComplex {module: self.module + other.module, arg: self.arg + other.arg}
    }
}

impl Mul<NumComplex> for ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: NumComplex) -> ExpComplex {
        let othermod = (other.real*other.real + other.imag*other.imag).sqrt();
        let otherarg = (other.real / othermod).acos();
        ExpComplex {module: self.module + othermod, arg: self.arg + otherarg}
    }
}

impl<'a, 'b> Mul<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn mul(self, other: &'b NumComplex) -> ExpComplex {
        let othermod = (other.real*other.real + other.imag*other.imag).sqrt();
        let otherarg = (other.real / othermod).acos();
        ExpComplex {module: self.module + othermod, arg: self.arg + otherarg}
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
        NumComplex {real: self.real*other.module*other.arg.cos() - self.imag*other.module*other.arg.sin(),
                    imag: self.real*other.module*other.arg.sin() + self.imag*other.module*other.arg.cos()}
    }
}

impl<'a, 'b> Mul<&'b ExpComplex> for &'a NumComplex {
    type Output = NumComplex;

    fn mul(self, other: &'b ExpComplex) -> NumComplex {
        NumComplex {real: self.real*other.module*other.arg.cos() - self.imag*other.module*other.arg.sin(),
                    imag: self.real*other.module*other.arg.sin() + self.imag*other.module*other.arg.cos()}
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
        let othermod = (other.real*other.real + other.imag*other.imag).sqrt();
        let otherarg = (other.real / othermod).acos();
        ExpComplex {module: self.module/othermod, arg: self.arg - otherarg}
    }
}

impl<'a, 'b> Div<&'b NumComplex> for &'a ExpComplex {
    type Output = ExpComplex;

    fn div(self, other: &'b NumComplex) -> ExpComplex {
        let othermod = (other.real*other.real + other.imag*other.imag).sqrt();
        let otherarg = (other.real / othermod).acos();
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
