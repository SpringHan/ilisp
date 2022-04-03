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

pub enum ParseTokenType {
    String,
    Symbol,
    Number
}

// The mutiple types of every token.
pub enum LispTokenType<'a> {
    EmptyCons,
    ArguType(u8),              // For the type of function's argument
    Cons(HashMap<u8, (&'a str, ParseTokenType)>, HashMap<u8, Box<LispToken<'a>>>, Vec<u8>),
    Property(HashMap<u8, (&'a str, ParseTokenType)>, HashMap<u8, Box<LispToken<'a>>>, Vec<u8>)
}

pub struct LispToken<'a> {
    value: LispTokenType<'a>,
    quoted: bool,
    line: u16
}

pub enum LispTokens<'a> {
    EmptyToken,                 // For storation after executing all the files
    Tokens(Vec<LispToken<'a>>)
}

impl<'a> LispToken<'a> {
    pub fn new(_type: u8, line_num: u16) -> LispToken<'a> {
        use self::LispTokenType::*;

        let child_value = match _type {
            0 => EmptyCons,
            1 => ArguType(0),
            2 => Cons(HashMap::new(), HashMap::new(), Vec::new()),
            3 => Property(HashMap::new(), HashMap::new(), Vec::new()),
            _ => panic!("The type of token's value is error!")
        };

        LispToken {
            value: child_value,
            quoted: false,
            line: line_num
        }
    }

    fn get_length(&self) -> u8 {
        match self.value {
            LispTokenType::Cons(item1, item2, _) | LispTokenType::Property(item1, item2, _) => {
                (item1.len() + item2.len()).try_into().unwrap()
            },
            _ => panic!("Failed to get the length of a item which has no child elements!")
        }
    }

    /// Set the quoted property of current token to true.
    // Used in (quote)
    pub fn set_to_quoted(&mut self) {
        self.quoted = true;
    }

    /// Change current Cons to EmptyCons.
    pub fn cons_to_empty(&mut self) {
        self.value = LispTokenType::EmptyCons;
    }

    /// Append `_value_str` or `_value_token` into the value of `LispToken`.
    /// When `starts_new_line` is true, add it into the Vector which is used to represent the newline
    pub fn append_element(&mut self, _value_str: (&'a str, ParseTokenType), _value_token: LispTokens<'a>, starts_new_line: bool) -> Result<(), ()> {
        match self.value {
            LispTokenType::Cons(ref mut item1, ref mut item2, ref mut item3)|
            LispTokenType::Property(ref mut item1, ref mut item2, ref mut item3) => {
                let new_index = self.get_length();
                match _value_token {
                    LispTokens::EmptyToken => {
                        item1.insert(new_index + 1, _value_str);
                    },
                    LispTokens::Tokens(t) => {
                        item2.insert(new_index + 1, Box::new(t[0]));
                    }
                }
                if starts_new_line {
                    item3.push(new_index);
                }
                Ok(())
            },

            _ => Err(())
        }
    }
}
