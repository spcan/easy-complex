//! Generates an `Angle` enum

/// Angle enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Angle<T> {
	/// Angle in degrees
	Degree(T),
	/// Angle in radians
	Radian(T),
}

macro_rules! impl_angle {
	([ $($float:ty),+ ]) => (
		$(
			impl Angle<$float> {
				pub fn degree(self) -> Angle<$float> {
					match self {
						Angle::Radian(a) => Angle::Degree(a*180.0/<$float>::from(common::f32::consts::PI)),
						Angle::Degree(b) => Angle::Degree(b),
					}
				}

				pub fn radian(self) -> Angle<$float> {
					match self {
						Angle::Radian(a) => Angle::Radian(a),
						Angle::Degree(b) => Angle::Radian(b*<$float>::from(common::f32::consts::PI)/180.0),
					}
				}
			}
		)+
	);
}