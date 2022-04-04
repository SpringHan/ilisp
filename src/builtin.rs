// Lisp Builtin functions.

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
use super::eval::env::LispType;
use super::eval::token::LispToken;

// BUG: Solve problem that make initialize of hashmap a const function.
// static mut LISP_BUILDTIN: HashMap<String, LispType> = hash_init();

// const fn hash_init() -> HashMap<String, LispType> {
//     HashMap::new()
// }

fn append_env<'a>(name: &'a str, env: LispType) {
    unsafe {
        LISP_BUILDTIN.insert(name.to_string(), env);
    }
}

/// Initialize builtins.
pub fn builtin_init() {
    // append_env("defn", LispType::BuiltinFunction());
    // append_env("defmacro", LispType::BuiltinFunction());
    // append_env("print", LispType::BuiltinFunction(|token: LispToken| -> LispType {
    // }))
}
