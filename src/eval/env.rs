// This file is the basic definitions about Environment.

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

use super::token::LispToken;

pub enum LispFunctionType<'a> {
    Builtin(fn(LispToken<'a>) -> LispType<'a>, u8),
    Lisp(LispToken<'a>)
}

pub enum LispType<'a> {
    Symbol(String),
    Boolean(bool),
    Number(String),
    LString(String),
    Cons(Vec<LispToken<'a>>), // Maybe it should be replaced
    Function(LispFunctionType<'a>, bool),   // The second argument is to check whether it's a func or macro
    Ref(String),
    Import(String),
    ThrowValue(&'a str, Box<LispType<'a>>)
}

impl<'a> LispType<'a> {
}
