// Life time module.
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

static LISP_LIFETIME: Vec<Vec<LispLifetime>> = Vec::new();

pub struct LispLifetime {
    independent: Option<String>,
    child_thread: Vec<usize>,
    value: HashMap<(String, bool), super::env::LispType>
}

impl LispLifetime {
    /// Init the main Lifetime for a certain module.
    pub fn init(module: &String) {
        unsafe {
            LISP_LIFETIME.push(Vec::new());
            LISP_LIFETIME.get_mut(0).unwrap().push(LispLifetime {
                independent: Some(module.to_string()),
                child_thread: Vec::new(),
                value: HashMap::new()
            });
        }
    }

    pub fn new_child(lifetime: usize) {
        unsafe {
            LISP_LIFETIME.get_mut(lifetime).unwrap().push(LispLifetime {
                independent: None,
                child_thread: Vec::new(),
                value: HashMap::new()
            })
        }
    }
}
