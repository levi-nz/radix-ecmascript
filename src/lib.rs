/*
 * Copyright (c) 2023 Levi-Michael Taylor. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * This library implements logic found in Google's open-source V8 library, specifically:
 *  double.h (https://github.com/v8/v8/blob/f83601408c3207211bc8eb82a8802b01fd82c775/src/numbers/double.h)
 *  DoubleToRadixCString (https://github.com/v8/v8/blob/f83601408c3207211bc8eb82a8802b01fd82c775/src/numbers/conversions.cc#L1269)
 * Copyright 2014, the V8 project authors. All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials provided
 *    with the distribution.
 *  Neither the name of Google Inc. nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * This license can also be found at: https://github.com/v8/v8/blob/f83601408c3207211bc8eb82a8802b01fd82c775/LICENSE
 */

//! `radix-ecmascript` adds a function, `to_radix_str`, to floating-point types (`f32` and `f64`)
//! to allow callers to obtain their radix string representation, just like in JavaScript,
//! in pure Rust. This library has no dependencies and is very lightweight.
//!
//! This library implements ECMAScript Language Specification Section 9.8.1,
//! "ToString Applied to the Number Type", and uses the same logic as found in
//! Google's open-source V8 engine.
//!
//! [DoubleToRadixCString](https://github.com/v8/v8/blob/f83601408c3207211bc8eb82a8802b01fd82c775/src/numbers/conversions.cc#L1269)
//! [Double utility](https://github.com/v8/v8/blob/f83601408c3207211bc8eb82a8802b01fd82c775/src/numbers/double.h)
//!
//! Example:
//! ```rust
//! use radix_ecmascript::{InvalidBaseError, ToRadixStr};
//!
//! fn main() {
//!     println!("{}", (0.123).to_radix_str(16).unwrap());
//! }
//! ```
//! This code prints `0.1f7ced916872b`, which can also be achieved by running
//! `(0.123).toString(16)` in JavaScript.
//!
//! This code unwraps the returned `Result`, but you should (probably) handle the
//! error in real cases. `to_radix_str` will only return `InvalidBaseError` if the
//! given `Base` is outside of the valid range, `MIN_BASE` and `MAX_BASE`.

mod f64_util;
mod tests;

use std::fmt::{Display, Formatter};

/// A floating-point base.
pub type Base = u8;

/// The minimum [Base] that can be passed into [ToRadixStr::to_radix_str].
pub const MIN_BASE: Base = 2;

/// The maximum [Base] that can be passed into [ToRadixStr::to_radix_str].
pub const MAX_BASE: Base = 36;

/// An error indicating that a given [Base] value is out of range of
/// [MIN_BASE] and [MAX_BASE].
#[derive(Debug)]
pub struct InvalidBaseError(Base);

impl Display for InvalidBaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid base: {}", self.0)
    }
}

impl std::error::Error for InvalidBaseError {}

/// Allows a type to be converted to radix string representation.
pub trait ToRadixStr: Sized {
    /// Returns the radix string representation of self using the functionality
    /// as defined in the ECMAScript Language Specification Section 9.8.1
    /// "ToString Applied to the Number Type".
    ///
    /// Returns [InvalidBaseError] if the given [Base] is out of range of
    /// [MIN_BASE] and [MAX_BASE] (inclusive).
    fn to_radix_str(self, base: Base) -> Result<String, InvalidBaseError>;
}

impl ToRadixStr for f64 {
    fn to_radix_str(self, base: Base) -> Result<String, InvalidBaseError> {
        use crate::f64_util::{exponent, next_float};

        // Validate base at runtime
        if !(MIN_BASE..=MAX_BASE).contains(&base) {
            return Err(InvalidBaseError(base));
        }

        // The result is always "NaN" if self is NaN.
        if self.is_nan() {
            return Ok("NaN".into());
        }

        // If self is +0 or -0, return "0".
        if self == 0.0 {
            return Ok("0".into());
        }

        // If self is +Infinity, return "Infinity".
        // If self is -Infinity, return "-Infinity".
        if self.is_infinite() {
            return Ok(if self.is_sign_positive() {
                "Infinity"
            } else {
                "-Infinity"
            }.into());
        }

        // Character array used for conversion.
        const CHARS: [char; 36] = [
            '0', '1', '2', '3', '4', '5',
            '6', '7', '8', '9', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n',
            'o', 'p', 'q', 'r', 's', 't',
            'u', 'v', 'w', 'x', 'y', 'z'
        ];

        // Temporary buffer for the result. We start with the decimal point in the
        // middle and write to the left for the integer part and to the right for the
        // fractional part. 1024 characters for the exponent and 52 for the mantissa
        // either way, with additional space for sign, decimal point and string
        // termination should be sufficient.
        const BUFFER_LEN: usize = 2200;
        // Allocate buffer and cursors.
        let mut buf: [char; BUFFER_LEN] = ['\0'; BUFFER_LEN];
        let mut int_cursor = BUFFER_LEN / 2;
        let mut fraction_cursor = int_cursor;

        // The value to reference and modify instead of self
        let value = self.abs();

        // Split the value into an integer part and a fractional part.
        let mut integer = value.floor();
        let mut fraction = value - integer;
        // We only compute fractional digits up to the input's precision.
        let mut delta = 0.5 * (next_float(value) - value);
        delta = delta.max(next_float(0.0));
        // Base as f64
        let base_f64 = base as f64;
        if fraction >= delta {
            // Insert decimal point.
            buf[fraction_cursor] = '.';
            fraction_cursor += 1;

            loop {
                // Shift up by one digit.
                fraction *= base_f64;
                delta *= base_f64;

                // Write digit.
                let digit = fraction as usize;
                buf[fraction_cursor] = CHARS[digit];
                fraction_cursor += 1;

                // Calculate remainder.
                fraction -= digit as f64;

                // Round to even.
                if (fraction > 0.5 || (fraction == 0.5 && (digit & 1) == 1)) && fraction + delta > 1.0 {
                    // We need to back trace already written digits in case of carry-over.
                    loop {
                        fraction_cursor -= 1;
                        if fraction_cursor == BUFFER_LEN / 2 {
                            // Carry over the integer part.
                            integer += 1.0;
                            break;
                        }

                        let c = buf[fraction_cursor];
                        // Reconstruct digit.
                        let digit = if c > '9' {
                            (c as u32) - ('a' as u32) + 10
                        } else {
                            (c as u32) - ('0' as u32)
                        };
                        if digit + 1 < base as u32 {
                            buf[fraction_cursor] = CHARS[digit as usize + 1];
                            fraction_cursor += 1;
                            break;
                        }
                    }

                    break;
                }

                if fraction < delta {
                    break;
                }
            }
        }

        // Compute integer digits. Fill unrepresented digits with zero.
        while exponent(integer / base_f64) > 0 {
            integer /= base_f64;
            int_cursor -= 1;
            buf[int_cursor] = '0';
        }

        loop {
            let remainder = integer % base_f64;
            int_cursor -= 1;
            buf[int_cursor] = CHARS[remainder as usize];
            integer = (integer - remainder) / base_f64;

            if integer <= 0.0 {
                break;
            }
        }

        // Add sign if negative.
        if self.is_sign_negative() {
            int_cursor -= 1;
            buf[int_cursor] = '-';
        }

        // Create result.
        let mut result = String::with_capacity(fraction_cursor - int_cursor);
        for c in &buf[int_cursor..fraction_cursor] {
            result.push(*c);
        }
        Ok(result)
    }
}

impl ToRadixStr for f32 {
    fn to_radix_str(self, base: Base) -> Result<String, InvalidBaseError> {
        (self as f64).to_radix_str(base)
    }
}
