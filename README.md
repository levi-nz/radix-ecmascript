# radix-ecmascript
Allows conversion to radix representation for floating-point types (`f32` and `f64`) in pure Rust,
just like in ECMAScript (`(0.5).toString(16)`, `(0.5).toString(36)` etc.).

This library implements ECMAScript Language Specification Section 9.8.1 "ToString Applied to the Number Type".

## Example
```rust
use radix_ecmascript::{InvalidBaseError, ToRadixStr};

fn main() -> Result<(), InvalidBaseError> {
    println!("{}", (0.123).to_radix_str(16)?);
}
```
This will print `0.1f7ced916872b`. The same can be achieved via calling `(0.123).toString(16)` in JavaScript.
