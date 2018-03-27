# easy-complex
[![Github build](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)]()
[![GitHub package version](https://img.shields.io/crates/v/easy_complex.svg?style=flat-square)](https://crates.io/crates/easy_complex)
[![Github license](https://img.shields.io/badge/license-apache-blue.svg?style=flat-square)](https://github.com/spcan/easy-complex/blob/master/LICENSE)

easy-complex is a no-dependencies crate that provides easy to use complex number operations and provides an explicit cast for all numeric types in the standard library.

[Github repo](https://github.com/spcan/easy-complex)

[crates.io page](https://crates.io/crates/easy_complex)

If you have any issues, please report them [here](https://github.com/spcan/easy-complex/issues)

## Features

  - Explicit cast from integers, unsigned integers and floats to Exponential Complex Number (ExpComplex).
  - Easy operations with overloaded standard operators.
  - A custom Display trait implementation that prints in either exponential form or coordinate form.
  - Compatibility with the num-traits crate through the feature "num_complex\_compatibility".

## Installation
easy complex doesn't have any dependencies except the standard library.
Just add to your ```Cargo.toml```

```
[dependencies]
easy_complex = "0.3.4"
```
If you don't use or want to use cargo, the crate can be found in this [crates.io](https://crates.io/crates/easy_complex) page

Versions before 0.3.1 have some errors, misspells and/or overcomplicated ways, their use is not recommended.


## Usage
For extensive explanation and usage go to the [wiki](https://github.com/spcan/easy-complex/wiki) where the full usage is demonstrated

To use the complex numbers do
```rust
use easy_complex::{EComplex, Complex};
```

To use the explicit conversion do
```rust
use easy_complex::ContainedInComplex;
```

 >This is based on the Real domain, which is contained within the Complex domain in Math

 >Warning!!! Values near zero may diverge in the argument if used in ExpComplex form

## TODO
 - [x] Extend compatibility
 - [ ] Add more examples and tests
 - [ ] Some advanced functions on the complex domain (**C**)
 - [ ] Clean up and more extensive documentation
 - [ ] Create a parser

## LICENSE
Apache License 2.0


