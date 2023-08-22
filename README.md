# radix-ecmascript
Allows conversion to radix representation for floating-point types (`f32` and `f64`) in pure Rust,
just like in ECMAScript (`(0.5).toString(16)`, `(0.5).toString(36)` etc.).

This library implements ECMAScript Language Specification Section 9.8.1 "ToString Applied to the Number Type".

## Example
```rust
use radix_ecmascript::{InvalidBaseError, ToRadixStr};

fn main() {
    println!("{}", (0.123).to_radix_str(16).unwrap());
}
```
This will print `0.1f7ced916872b`. The same can be achieved via calling `(0.123).toString(16)` in JavaScript.

Note in this example that we unwrap the `Result<String, InvalidBaseError>`. In a real case, you should *probably*
have proper error propagation, however it's good to know that `to_radix_str` will only return an `InvalidBaseError`
if the given base is outside the valid range (`radix_ecmascript::MIN_BASE` and `radix_ecmascript::MAX_BASE`),
so if you're passing in a constant you can safely unwrap the error.
