# easy-complex
[![Github build](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)]()
[![GitHub package version](https://img.shields.io/crates/v/easy_complex.svg?style=flat-square)](https://crates.io/crates/easy_complex)
[![Github license](https://img.shields.io/badge/license-apache-blue.svg?style=flat-square)](https://github.com/spcan/easy-complex/blob/master/LICENSE-APACHE)
[![Github license](https://img.shields.io/badge/license-mit-blue.svg?style=flat-square)](https://github.com/spcan/easy-complex/blob/master/LICENSE-MIT)

`easy-complex` is a no-dependencies crate that provides easy to use complex number operations and provides casting for most numeric types in the standard library.

[Github repo](https://github.com/spcan/easy-complex)

[crates.io page](https://crates.io/crates/easy_complex)

If you have any issues, please report them [here](https://github.com/spcan/easy-complex/issues)

## Features

  - Explicit cast from integers, unsigned integers and floats to Complex number (`Complex`) and Exponential Complex number (`EComplex`).
  - Easy operations with overloaded standard operators.
  - A custom Display trait implementation that prints in either exponential form or rectangular form.

## Usage
easy complex doesn't have any dependencies except the standard library.
Just add to your ```Cargo.toml```

```
[dependencies]
easy_complex = "0.4.0"
```

Add to your crate root:
```
extern crate easy_complex;
```

If you don't use or want to use cargo, the crate can be found in this [crates.io](https://crates.io/crates/easy_complex) page

For extensive explanation and usage go to the [wiki](https://github.com/spcan/easy-complex/wiki) where the full usage is demonstrated


## TODO
 - [ ] Extend compatibility (On hold)
 - [ ] Add more examples and tests
 - [x] Some advanced functions on the complex domain (**C**)
 - [x] Clean up and more extensive documentation
 - [ ] Create a parser (On hold)

## Guide
As the crate is generated by macros, documentation is pretty much impossible to create.

Below you will find a summary of the crate's structs and methods.

| Struct | Description |
| -------- | ---------- |
| `Complex` | Complex number represented in rectangular form |
| `EComplex` | Complex number represented in exponential form |


| Enum | Description | Variants |
| -------- | -------- | --- |
| Angle | An angle | `Radians`, `Degrees` |


These methods are available in both `Complex` and `EComplex`

| Method | Arguments | Output | Description |
| -------- | ---------- | -------- | ---------- |
| new | `f32, f32` or `f64, f64` | `Self` | Generates a new struct |
| real |  | `f32` or `f64` | Returns the value of the real part of the complex number |
| imag |  | `f32` or `f64` | Returns the value of the imaginary part of the complex number |
| module |  | `f32` or `f64` | Returns the value of the module of the complex number |
| arg |  | `f32` or `f64` | Returns the value of the argument of the complex number |
| conjugate |  | `Self` | Returns the conjugate of the complex number |
| ln |  | `Self` | Returns the natural logarithm of the complex number |
| log | `f32` or `f64` | `Self` | Returns the logarithm in the given base | 
| log2 |  | `Self` | Returns the logarithm in base 2 | 
| log10 |  | `Self` | Returns the logarithm in base 10 | 
| powf | `f32` or `f64` | `Self` | Returns the number to the given power |
| exp |  | `Self` | Returns `e^self` |
| sqrt|  | `Self` | Returns the square root of the number |
| root | `usize` | `Vec<Self>` | Returns a `Vec` with all the nth roots of the number |
| powc | `Self` | `Self` | Returns `self` to the given complex power |
| expf | `f32` or `f64` | `Self` | Returns the given base to the `self` complex power |
| cos, sin, tan |  |  `Self` | Returns the corresponding geometrical function |
| cosh, sinh, tanh |  |  `Self` | Returns the corresponding hyperbolic geometrical function |

## How to make automatic conversion

To do automatic conversion, use the `convert!` macro in the `convert` module.

This macro takes a list of types that cast into either `f32` or `f64` and implements the corresponding methods for the given struct.

This example is taken directly from the source code of the crate:
```
convert!([i8, u8, i16, u16, f32], f32, Complex);
```

## LICENSE
Dual licensed under Apache 2.0 and MIT licenses

