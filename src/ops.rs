//! Generates all the operations needed

macro_rules! add_op {
	($float:ty, Complex) => {
		impl<A> common::ops::Add<A> for Complex<$float>
			where A: Into<Complex<$float>>
		{
			type Output = Complex<$float>;

			fn add(self, other: A) -> Complex<$float> {
				let c: Complex<$float> = other.into();
				Self {
					real: self.real + c.real,
					imag: self.imag + c.imag,
				}
			}
		}
	};

	($float:ty, EComplex) => {
		impl<A> common::ops::Add<A> for EComplex<$float>
			where A: Into<Complex<$float>>
		{
			type Output = Self;

			fn add(self, other: A) -> Self {
				let c: Complex<$float> = other.into();
				let tmp: Complex<$float> = self.into();

				let new: Complex<$float> = tmp + c;

				new.into()
			}
		}
	};
}

macro_rules! sub_op {
	($float:ty, Complex) => (
		impl<A> common::ops::Sub<A> for Complex<$float>
			where A: Into<Complex<$float>>
		{
			type Output = Complex<$float>;

			fn sub(self, other: A) -> Complex<$float> {
				let c: Complex<$float> = other.into();
				Self {
					real: self.real - c.real,
					imag: self.imag - c.imag,
				}
			}
		}
	);

	($float:ty, EComplex) => {
		impl<A> common::ops::Sub<A> for EComplex<$float>
			where A: Into<Complex<$float>>
		{
			type Output = Self;

			fn sub(self, other: A) -> Self {
				let c: Complex<$float> = other.into();

				let tmp: Complex<$float> = self.into();

				let new: Complex<$float> = tmp + c;

				new.into()
			}
		}
	};
}

macro_rules! mul_op {
	($float:ty, Complex) => (
		impl<A> common::ops::Mul<A> for Complex<$float>
			where A: Into<Complex<$float>>
		{
			type Output = Complex<$float>;

			fn mul(self, other: A) -> Complex<$float> {
				let c: Complex<$float> = other.into();
				Self {
					real: self.real * c.real - self.imag * c.imag,
					imag: self.imag * c.real + self.real * c.imag,
				}
			}
		}
	);

	($float:ty, EComplex) => {
		impl<A> common::ops::Mul<A> for EComplex<$float>
			where A: Into<EComplex<$float>>
		{
			type Output = Self;

			fn mul(self, other: A) -> Self {
				let c: Self = other.into();

				Self {
					module: self.module * c.module,
					arg: self.arg + c.arg,
				}
			}
		}
	};
}

macro_rules! div_op {
	($float:ty, Complex) => (
		impl<A> common::ops::Div<A> for Complex<$float>
			where A: Into<Complex<$float>>
		{
			type Output = Complex<$float>;

			fn div(self, other: A) -> Complex<$float> {
				let c: Complex<$float> = other.into();
				Self {
					real: (self.real*c.real + self.imag*c.imag) / (c.real*c.real + c.imag*c.imag),
					imag: (self.imag*c.real - self.real*c.imag) / (c.real*c.real + c.imag*c.imag),
				}
			}
		}
	);

	($float:ty, EComplex) => {
		impl<A> common::ops::Div<A> for EComplex<$float>
			where A: Into<EComplex<$float>>
		{
			type Output = Self;

			fn div(self, other: A) -> Self {
				let c: Self = other.into();

				Self {
					module: self.module / c.module,
					arg: self.arg - c.arg,
				}
			}
		}
	};
}

macro_rules! neg_op {
	($float:ty, Complex) => (
		impl common::ops::Neg for Complex<$float> {
			type Output = Complex<$float>;

			fn neg(self) -> Complex<$float> {
				Self {
					real: -self.real,
					imag: -self.imag,
				}
			}
		}
	);

	($float:ty, EComplex) => {
		impl common::ops::Neg for EComplex<$float> {
			type Output = EComplex<$float>;

			fn neg(self) -> EComplex<$float> {
				let mut narg: $float = self.arg;

				if self.arg.is_sign_positive() {
					narg -= <$float>::from(common::f32::consts::PI)
				} else {
					narg += <$float>::from(common::f32::consts::PI)
				};
			
				Self {
					module: self.module,
					arg: narg,
				}
			}
		}
	};
}


macro_rules! gen_ops {
	( $float:ty, $ctype:ident) => (
		add_op!($float, $ctype);
		sub_op!($float, $ctype);
		mul_op!($float, $ctype);
		div_op!($float, $ctype);
		neg_op!($float, $ctype);
	);
}
