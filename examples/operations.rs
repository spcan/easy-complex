extern crate easy_complex;

use easy_complex::{ExpComplex, NumComplex};

fn main() {
    let complex1 = ExpComplex {module: 2.0, arg: 1.0};
    let complex2 = NumComplex {real: 2.3, imag: 5.2};

    //No cast between types needed
    //Output type is always the leftmost complex number, in this case ExpComplex
    println!("{}", complex1*complex2);

    //Negative values can be added as well
    println!("{}", -complex2);

    //Conjugate calculation
    println!("{}", complex2.conjugate());

    //And the conjugate
    println!("{}", complex2.conjugate() + complex1.conjugate());

}
