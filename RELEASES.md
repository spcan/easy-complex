## Version 0.4.0
 - Added more tests
 - Method optimization
 - Added `log2()` and `log10()` methods
 - Deleted `euler_id` function

### Breaking changes
 - Deleted traits `Zero` and `One`
 - Changed the explicit casting
    Now explicit casting is separated into the `complex()` method, which gives a `Complex` number, and the `ecomplex()` method which gives an `EComplex` number


## Version 0.3.4
 - Add `real()`, `imag()`, `module()` and `arg()`
methods to both structs
 - Add `tanh()`
 - Add some tests
 - Update and expand documentation

### Breaking changes
 - Change struct names to **Complex** and **EComplex**
 - Shorten function names


## Version 0.3.3
 - Added `exp()`, `log()` and `tan()` functions
 - Implement inequality
 - Implement `Zero` and `One` traits (nightly only)
 - Add compatibility with **num** crate
 - Fixed "+" sign not displaying correctly on format

### Breaking changes
 - Separated `pow()` into `powi()` and `powf()`