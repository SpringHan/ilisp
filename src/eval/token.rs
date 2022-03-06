// This is the token file for Ideal Lisp.

// Copyright (c) 2022 SpringHan

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
// OR OTHER DEALINGS IN THE SOFTWARE.

use std::collections::HashMap;

// The mutiple types of every token.
pub enum LispTokenType<'a> {
    EmptyCons,
    ArguType(u8),              // For the type of function's argument
    Cons(HashMap<u8, &'a str>, HashMap<u8, Box<LispToken<'a>>>, Vec<u8>),
    Property(HashMap<u8, &'a str>, HashMap<u8, Box<LispToken<'a>>>, Vec<u8>)
}

pub struct LispToken<'a> {
    car: &'a str,
    value: LispTokenType<'a>,
    prefix: bool,               // If the car is a prefix alias
    line: u16
}

pub enum LispTokens<'a> {
    EmptyToken,
    Tokens(Vec<LispToken<'a>>)
}

impl<'a> LispToken<'a> {
    pub fn new(_car: &str, _type: u8, line_num: u16, is_prefix: bool) -> LispToken {
        use self::LispTokenType::*;

        let child_value = match _type {
            0 => EmptyCons,
            1 => ArguType(0),
            2 => Cons(HashMap::new(), HashMap::new(), Vec::new()),
            3 => Property(HashMap::new(), HashMap::new(), Vec::new()),
            _ => panic!("The type of token's value is error!")
        };

        LispToken{
            car: _car,
            value: child_value,
            prefix: is_prefix,
            line: line_num
        }
    }
}
