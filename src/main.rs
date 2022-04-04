// ILisp ---- Ideal Lisp

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

pub mod eval;
pub mod builtin;

use eval::module;

fn main() {
    module::LispLibrary::new("std", "");
    let a = module::LispLibrary::find("std");
    match a {
        Some(b) => println!("{}", b.get_prop(true)),
        None => ()
    }
    // let mut c: Vec<eval::token::LispToken> = Vec::new();
    // c.push(eval::token::LispToken::new("a", 0, 0, false));
    // module::LispModule::new("std", "main", "", true, c);
    // eval::parse::parse("(defalias #' func)\n(print \"haha\"\n)".to_string());
    // eval::parse::parse("(print '(a 'a))".to_string());
    // TODO: The next next step: Add repl & builtin functions
    // let mut c: Vec<eval::token::LispToken> = Vec::new();
    let c = eval::parse::parse("(print '(a 'b))".to_string());
    println!("{:?}", c);
}
