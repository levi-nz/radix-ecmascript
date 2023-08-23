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

#[cfg(test)]
use crate::*;

#[test]
fn test_to_radix_str() {
    let x = 0.05217266072382676;

    const EXPECTED: [&str; 35] = [
        "0.00001101010110110010111111111111100011011101110000011",
        "0.00110200022020012220120101220020111",
        "0.003111230233333320313130012",
        "0.0112300443304443032401333",
        "0.015134100114141415452",
        "0.023616030022055516622",
        "0.032554577770673406",
        "0.042026618651180643",
        "0.05217266072382676",
        "0.06349506198377117",
        "0.0761a289186a5058",
        "0.08a814611c1a0636",
        "0.0a3239cda0bc978",
        "0.0bb139330b51876",
        "0.0d5b2fff8ddc18",
        "0.0f158c3647753f",
        "0.0gg4fe405e39cb",
        "0.0ifg3cei7ia9h",
        "0.10h7ca5eabje8",
        "0.1203c8f80ka74",
        "0.135bgf5hk3ll2",
        "0.14di134k07bii",
        "0.1615f6hcdi284",
        "0.17f4nfon3e1ie",
        "0.196pgpp81pf5f",
        "0.1b0oihjj5ijc",
        "0.1cp86j9br2n3",
        "0.1epcl695l16b",
        "0.1gsjpjk0r2dp",
        "0.1j48gudjoqjq",
        "0.1ldivvsdrgc",
        "0.1nqulj60i8vr",
        "0.1qak6wn4rq5o",
        "0.1svvkxr2efv",
        "0.1vm61aaabtc",
    ];

    for base in MIN_BASE..=MAX_BASE {
        assert_eq!(x.to_radix_str(base).unwrap(), EXPECTED[base as usize - 2]);
    }
}

#[test]
fn test_to_radix_str_ranges() {
    // Valid ranges
    for base in MIN_BASE..=MAX_BASE {
        assert!((0.0).to_radix_str(base).is_ok());
    }

    // Invalid ranges
    assert!((0.0).to_radix_str(MIN_BASE-1).is_err());
    assert!((0.0).to_radix_str(MAX_BASE+1).is_err());
}
