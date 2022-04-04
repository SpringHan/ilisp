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

#[derive(Debug)]
pub enum ParseTokenType {
    String,
    Symbol,
    Number
}

// The mutiple types of every token.
#[derive(Debug)]
pub enum LispTokenType {
    EmptyCons,
    SoloEle(String, ParseTokenType, u16),
    ArguType(u8),              // For the type of function's argument
    Cons(Vec<Box<LispToken>>),
    Property(Vec<Box<LispToken>>)
}

#[derive(Debug)]
pub struct LispToken {
    value: LispTokenType,
    quoted: bool
}

#[derive(Debug)]
pub enum LispTokens {
    EmptyToken,                 // For storation after executing all the files
    Tokens(Vec<LispToken>)
}

impl LispToken {

    /// The function for initialize the token variale.
    pub fn init() -> LispToken {
        LispToken {
            value: LispTokenType::EmptyCons,
            quoted: false
        }
    }

    pub fn new_solo<'a>(name: &'a String, token_type: ParseTokenType, line: u16) -> LispToken {
        LispToken {
            value: LispTokenType::SoloEle(name.to_owned(), token_type, line),
            quoted: false,
        }
    }

    pub fn new(_type: u8) -> LispToken {
        use self::LispTokenType::*;

        let child_value = match _type {
            0 => EmptyCons,
            1 => ArguType(0),
            2 => Cons(Vec::new()),
            3 => Property(Vec::new()),
            _ => panic!("The type of token's value is error!")
        };

        LispToken {
            value: child_value,
            quoted: false
        }
    }

    fn get_length(&self) -> u8 {
        match &self.value {
            LispTokenType::Cons(item) | LispTokenType::Property(item) => {
                item.len().try_into().unwrap()
            },
            _ => panic!("Failed to get the length of a item which has no child elements!")
        }
    }

    /// Set the quoted property of current token to true.
    // Used in (quote)
    pub fn set_to_quoted(&mut self) {
        self.quoted = true;
    }

    /// Append `_value_str` or `_value_token` into the value of `LispToken`.
    /// When `starts_new_line` is true, add it into the Vector which is used to represent the newline
    pub fn append_element(&mut self, _value_str: Option<(String, ParseTokenType, u16)>, _value_token: Option<LispToken>) -> Result<(), ()> {
        let new_index = self.get_length();
        match &mut self.value {
            LispTokenType::Cons(ref mut item) | LispTokenType::Property(ref mut item) => {
                match _value_token {
                    None => {
                        if let Some((v, t, l)) = _value_str {
                            item.push(Box::new(LispToken::new_solo(&v, t, l)));
                        }
                    },
                    Some(t) => {
                        item.push(Box::new(t));
                    }
                }

                Ok(())
            },

            _ => Err(())
        }
    }

    pub fn get_element(&self, index: usize) -> &Box<LispToken> {
        match &self.value {
            LispTokenType::Cons(ref item) |
            LispTokenType::Property(ref item) => {
                item.get(index).unwrap()
            },

            _ => panic!("Error in getting element of LispToken.")
        }
    }
}
