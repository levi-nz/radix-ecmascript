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

//! Helper functions for 64-bit floats, implemented from Google's open-source V8 engine.
//!
//! [V8 Source Code](https://github.com/v8/v8/blob/f83601408c3207211bc8eb82a8802b01fd82c775/src/numbers/double.h)

const K_SIGN_MASK: u64 = 0x8000_0000_0000_0000;

const K_EXPONENT_MASK: u64 = 0x7FF0_0000_0000_0000;

const K_SIGNIFICAND_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;

const K_HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;

const K_INFINITY: u64 = 0x7FF0_0000_0000_0000;

const K_PHYSICAL_SIGNIFICAND_SIZE: i32 = 52; // Excludes hidden bit.

const K_EXPONENT_BIAS: i32 = 0x3FF + K_PHYSICAL_SIGNIFICAND_SIZE;
const K_DENORMAL_EXPONENT: i32 = -K_EXPONENT_BIAS + 1;

/// Reports if the given floating-point bits is subnormal.
fn is_denormal(bits: u64) -> bool {
    bits & K_EXPONENT_MASK == 0
}

/// Gets the significand of the given floating-point bits.
pub fn significand(bits: u64) -> u64 {
    let significand = bits & K_SIGNIFICAND_MASK;

    if is_denormal(bits) {
        significand
    } else {
        significand + K_HIDDEN_BIT
    }
}

/// Reports if the given floating-point bits is positive.
fn is_pos(bits: u64) -> bool {
    bits & K_SIGN_MASK == 0
}

/// Returns the next greater f64.
/// Returns +Infinity if f == +Infinity.
pub(crate) fn next_float(f: f64) -> f64 {
    let bits = f.to_bits();

    if bits == K_INFINITY {
        return f;
    }

    let is_neg = !is_pos(bits);
    if is_neg && significand(bits) == 0 {
        return 0.0;
    }

    f64::from_bits(if is_neg {
        bits - 1
    } else {
        bits + 1
    })
}

/// Gets the exponent of f.
pub(crate) fn exponent(f: f64) -> i32 {
    let bits = f.to_bits();

    if is_denormal(bits) {
        return K_DENORMAL_EXPONENT;
    }

    let biased = (bits & K_EXPONENT_MASK >> K_PHYSICAL_SIGNIFICAND_SIZE) as i32;
    biased - K_EXPONENT_BIAS
}
