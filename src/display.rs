//! Generates the display of the given struct

macro_rules! gen_display {
	(Complex, [ $($float:ty),+ ]) => (
		$(
		impl common::fmt::Display for Complex<$float> {
			fn fmt(&self, f: &mut common::fmt::Formatter) -> common::fmt::Result {
				write!(f, "{} {} {}j", self.real, if self.imag.is_sign_positive() {"+"} else {"-"}, self.imag)
			}
		}
		)+
	);

	(EComplex, [ $($float:ty),+ ]) => (
		$(
		impl common::fmt::Display for EComplex<$float> {
			fn fmt(&self, f: &mut common::fmt::Formatter) -> common::fmt::Result {
				write!(f, "{} · e^({}j)", self.module, self.arg)
			}
		}
		)+
	);
}
