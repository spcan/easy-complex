# easy-complex
[![Github build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![GitHub package version](https://img.shields.io/badge/crates.io-0.3.1-orange.svg)](https://crates.io/crates/easy_complex)
[![Github license](https://img.shields.io/badge/license-apache-blue.svg)](https://github.com/spcan/easy-complex/blob/master/LICENSE)

easy-complex is a no-dependencies crate that provides easy to use complex number operations and provides an explicit cast for all numeric types in the standard library.

[Github repo](https://github.com/spcan/easy-complex)

[crates.io page](https://crates.io/crates/easy_complex)

## Features

  - Explicit cast from integers, unsigned integers and floats to Exponential Complex Number (ExpComplex).
  - Easy operations with overloaded standard operators.
  - Automatic conversion to the leftmost type of complex in operations.
  - A custom Display trait implementation that prints in either exponential form or coordinate form.

## Installation
easy complex doesn't have any dependencies except the standard library.
Just add to your ```Cargo.toml```

```
[dependencies]
easy_complex = "0.3.2"
```
If you don't use or want to use cargo, the crate can be found in this [crates.io](https://crates.io/crates/easy_complex) page

Versions before 0.3.1 have some errors, misspells and/or overcomplicated ways, their use is not recommended.


## Usage
For extensive explanation go to the [wiki](https://github.com/spcan/easy-complex/wiki)

To use the complex numbers do
```
use easy_complex::{ExpComplex, NumComplex};
```

To use the explicit conversion do
```
use easy_complex::ContainedInComplex;
```

 >This is based on the Real domain, which is contained within the Complex domain in Math

 >Warning!!! Values near zero may diverge in the argument if used in ExpComplex form

## TODO
 - ~~Implement destructuring for the complex numbers as well as assigning from tuples~~
 - Some advanced functions on the complex domain (**C**)
 - Clean up and more extensive documentation

## LICENSE
Apache License 2.0

