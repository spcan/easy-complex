//! Code generation macros

/// Basic methods
macro_rules! basic {
	(Complex, [ $($f:ty),+ ]) => (
		$(
		impl Complex<$f> {
			pub fn new(real: $f, imag: $f) -> Complex<$f> {
				Self {
					real,
					imag,
				}
			}

			pub fn real(&self) -> $f {
				self.real
			}

			pub fn imag(&self) -> $f {
				self.imag
			}

			pub fn module(&self) -> $f {
				(self.real*self.real + self.imag*self.imag).sqrt()
			}

			pub fn arg(&self) -> Angle<$f> {
				match self.real {
					a if a == 0.0 => match self.imag.is_sign_negative() {
						true => Angle::Radian((-common::f32::consts::PI / 2.0).into()),
						_ => Angle::Radian((common::f32::consts::PI / 2.0).into()),
					}
					_ => Angle::Radian((self.imag/self.real).atan()),
				}
			}

			pub fn conjugate(&self) -> Self {
				Complex {
					real:  self.real,
					imag: -self.imag,
				}
			}
		}
		)+
	);

	(EComplex, [ $($f:ty),+ ]) => (
		$(
		impl EComplex<$f> {
			pub fn new(module: $f, arg: $f) -> EComplex<$f> {
				Self {
					module,
					arg,
				}
			}

			pub fn real(&self) -> $f {
				self.module * self.arg.cos()
			}

			pub fn imag(&self) -> $f {
				self.module * self.arg.sin()
			}

			pub fn module(&self) -> $f {
				self.module
			}

			pub fn arg(&self) -> Angle<$f> {
				Angle::Radian(self.arg)
			}

			pub fn conjugate(&self) -> Self {
				EComplex {
					module: self.module,
					arg: -self.arg,
				}
			}
		}
		)+
	);
}

macro_rules! math {
	(Complex, [ $($f:ty),+ ]) => (
		$(
			impl Complex<$f> {
				pub fn ln(&self) -> Self {
					Complex { real: self.module(), imag: match self.arg() {
						Angle::Radian(a) => a,
						_ => unreachable!(),
					}}
				}

				pub fn log(&self, base: $f) -> Self {
					let narg = match self.arg() {
						Angle::Radian(a) => a,
						_ => unreachable!(),
					};

					Complex { real: self.module().log(<$f>::from(base)), imag: narg / <$f>::from(base.ln()) }
				}

				pub fn powf(&self, power: f32) -> Self {
					let out: EComplex<$f> = self.into();
					<Complex<$f>>::from(out.powf(power))
				}

				pub fn exp(&self) -> Self {
					Complex { real: self.real.exp() * self.imag.cos(), imag: self.real.exp() * self.imag.sin() }
				}

				pub fn sqrt(&self) -> Vec<Self> {
					self.root(2)
				}

				pub fn root(&self, r: usize) -> Vec<Self> {
					match r {
						0 | 1 => Vec::new(),
						_ => {
							let mut out = Vec::new();
							
							for c in <EComplex<$f>>::from(self).root(r) {
								out.push(<Complex<$f>>::from(c))
							}

							out
						},
					}
				}

				pub fn powc(&self, power: Complex<$f>) -> Self {
					<Complex<$f>>::from(<EComplex<$f>>::from(self).powc(power.into()))
				}

				pub fn expf(&self, base: $f) -> Self {
					<Complex<$f>>::from(<EComplex<$f>>::from(self).expf(base))
				}
			}
		)+
	);

	(EComplex, [ $($f:ty),+ ]) => (
		$(
			impl EComplex<$f> {
				pub fn ln(&self) -> Self {
					<EComplex<$f>>::from(Complex { real: self.module, imag: self.arg })
				}

				pub fn log(&self, base: $f) -> Self {
					<EComplex<$f>>::from(Complex { real: self.module.log(<$f>::from(base)), imag: self.arg / <$f>::from(base.ln()) })
				}

				pub fn powf(&self, power: f32) -> Self {
					EComplex {
						module: self.module.powf(power.into()),
						arg: self.arg * <$f>::from(power),
					}
				}

				pub fn exp(&self) -> Self {
					<EComplex<$f>>::from(<Complex<$f>>::from(self).exp())
				}

				pub fn sqrt(&self) -> Vec<Self> {
					self.root(2)
				}

				pub fn root(&self, r: usize) -> Vec<Self> {
					match r {
						0 | 1 => Vec::new(),
						_ => {
							let mut out = Vec::new();
							let module: $f = self.module.powf(1.0 / (r as $f));

							for i in 0..r {
								out.push(
									EComplex {
										module: module,
										arg: (self.arg * 2.0 * <$f>::from(common::f32::consts::PI) * (i as $f) ) / (r as $f),
									}
								)
							}

							out
						},
					}
				}

				pub fn powc(&self, power: EComplex<$f>) -> Self {
					let exp: Complex<$f> = power.into();

					EComplex {
						module: self.module.powf(exp.real) * (-exp.imag * self.arg).exp(),
						arg: exp.real * self.arg + exp.imag * self.module.ln(),
					}
				}

				pub fn expf(&self, base: $f) -> Self {
					EComplex {
						module: base.powf(self.real()),
						arg: self.imag() * base.ln(),
					}
				}

			}
		)+
	);
}

macro_rules! geometry {
	(Complex, [ $($f:ty),+]) => (
		$(
			impl Complex<$f> {
				pub fn cosh(&self) -> Self {
					Complex {
						real: self.real.cosh() * self.imag.cos(),
						imag: self.real.sinh() * self.imag.sin(),
					}
				}

				pub fn sinh(&self) -> Self {
					Complex {
						real: self.real.sinh() * self.imag.cos(),
						imag: self.real.cosh() * self.imag.sin(),
					}
				}

				pub fn tanh(&self) -> Self {
					let a = Complex { real: self.real.tanh(), imag: self.imag.tan() };
					let b = Complex { real: 1.0, imag: self.real.tanh()*self.imag.tan() };
					a / b
				}

				pub fn cos(&self) -> Self {
					Self {
						real:  self.real.cos() * self.imag.cosh(),
						imag: -self.real.sin() * self.imag.sinh(),
					}
				}

				pub fn sin(&self) -> Self {
					Self {
						real:  self.real.sin() * self.imag.cosh(),
						imag: -self.real.cos() * self.imag.sinh(),
					}
				}

				pub fn tan(&self) -> Self {
					let out: Complex<$f> = Complex {
						real: (self.real * 2.0).sin(),
						imag: (self.imag * 2.0).sinh() / ((self.real * 2.0).cos() + (self.imag * 2.0).cos())
					};
					out
				}

			}
		)+
	);

	(EComplex, [ $($f:ty),+]) => (
		$(
			impl EComplex<$f> {
				pub fn cosh(&self) -> Self {
					let out: Complex<$f> = self.into();
					<EComplex<$f>>::from(out.cosh())
				}

				pub fn sinh(&self) -> Self {
					let out: Complex<$f> = self.into();
					<EComplex<$f>>::from(out.sinh())
				}

				pub fn tanh(&self) -> Self {
					let out: Complex<$f> = self.into();
					<EComplex<$f>>::from(out.tanh())
				}

				pub fn cos(&self) -> Self {
					let out: Complex<$f> = self.into();
					<EComplex<$f>>::from(out.cos())
				}

				pub fn sin(&self) -> Self {
					let out: Complex<$f> = self.into();
					<EComplex<$f>>::from(out.sin())
				}

				pub fn tan(&self) -> Self {
					let out: Complex<$f> = self.into();
					<EComplex<$f>>::from(out.tan())
				}
			}
		)+
	);
}