# easy-complex

easy-complex is a no dependencies crate that provides easy to use complex number operations and provides an explicit cast for all numeric types in the standard library.
## Features

  - Explicit cast from integers, unsigned integers and floats to Exponential Complex Number (ExpComplex).
  - Easy operations with overloaded standard operators.
  - Automatic conversion to the leftmost type of complex in operations.
  - A custom Display trait implementation that prints according to standards.

## TODO
 - Implement destructuring for the complex numbers as well as assigning from tuples
 - Some advanced functions on the complex realm (**C**)
 - Clean up and more extensive documentation

### Installation
easy complex doesn't have any dependencies except the standard library.
Just add to your ```Cargo.toml```

```
[dependencies]
easy_complex = "0.2.0"
```

## LICENSE
Apache License 2.0
