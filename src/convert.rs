//! Conversion macros

/// This macro performs a conversion from any type that can cast
/// into a float (either `f32` or `f64`) and transforms it into a
/// Either `Complex` or `EComplex`
/// 
/// If a type `T` is given, it is treated as a `real` only value.
/// If the type `(T, T)` is given, it maps the struct's fields directly to it
#[macro_export]
macro_rules! convert {
	([ $($other:ty),+ ], $float:ty, $ctype:ident) => {
		$(
			impl From<$other> for $ctype<$float> {
				fn from(other: $other) -> Self {
					Self::new(other.into(), Default::default())
				}
			}

			impl From<($other, $other)> for $ctype<$float> {
				fn from(other: ($other, $other)) -> Self {
					Self::new(other.0.into(), other.0.into())
				}
			}
		)+
	};
}

macro_rules! dual_convert {
	($c1:ident, $c2:ident) => (
		dc_inner!(f32, f32, $c1, $c2);
		dc_inner!(f32, f64, $c1, $c2);
		dc_inner!(f64, f64, $c1, $c2);

		dc_inner!(f32, f32, $c2, $c1);
		dc_inner!(f32, f64, $c2, $c1);
		dc_inner!(f64, f64, $c2, $c1);
	);
}

macro_rules! dc_inner {
	($t1:ty, $t2:ty, Complex, EComplex) => (
		impl From<Complex<$t1>> for EComplex<$t2> {
			fn from(other: Complex<$t1>) -> EComplex<$t2> {
				EComplex {
					module: other.module().into(),
					arg: match other.arg() {
						Angle::Radian(a) => a.into(),
						_ => unreachable!(),
					},
				}
			}
		}

		impl<'a> From<&'a Complex<$t1>> for EComplex<$t2> {
			fn from(other: &'a Complex<$t1>) -> EComplex<$t2> {
				EComplex {
					module: other.module().into(),
					arg: match other.arg() {
						Angle::Radian(a) => a.into(),
						_ => unreachable!(),
					},
				}
			}
		}
	);

	($t1:ty, $t2:ty, EComplex, Complex) => (
		impl From<EComplex<$t1>> for Complex<$t2> {
			fn from(other: EComplex<$t1>) -> Complex<$t2> {
				Complex {
					real: other.real().into(),
					imag: other.imag().into(),
				}
			}
		}

		impl<'a> From<&'a EComplex<$t1>> for Complex<$t2> {
			fn from(other: &'a EComplex<$t1>) -> Complex<$t2> {
				Complex {
					real: other.real().into(),
					imag: other.imag().into(),
				}
			}
		}
	);
}
