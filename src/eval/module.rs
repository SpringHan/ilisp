// The file for module of ILisp.

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

static mut LISP_MODULES: Vec<LispLibrary> = Vec::new();

pub struct LispModuleBasic<T> {
    name: String,
    definition: String,
    public: T,
    private: T
}

pub struct LispModule<'a> {
    basic: LispModuleBasic<HashMap<String, super::env::LispType<'a>>>,
    modules: Vec<String>,
    tokens: super::token::LispTokens<'a>
}

pub struct LispLibrary<'a> {
    basic: LispModuleBasic<Vec<LispModule<'a>>>
}

pub trait LispModules {
}

impl<'a> LispLibrary<'a> {
    pub fn new(_name: &str, def: &str) {
        unsafe {
            LISP_MODULES.push(LispLibrary{
                basic: LispModuleBasic{
                    name: _name.to_string(),
                    definition: def.to_string(),
                    public: Vec::new(),
                    private: Vec::new()
                }
            });
        }
    }

    pub fn find(lib: &'static str) -> Option<&mut LispLibrary> {
        let mut result: Option<&mut LispLibrary> = None;
        unsafe {
            for e in LISP_MODULES.iter_mut() {
                if &e.basic.name == lib {
                    result = Some(e)
                }
            }}
        result
    }
}

impl<'a> LispModule<'a> {
    pub fn new(_lib: &'static str, _name: &str, def: &str, tokens: Vec<super::token::LispToken<'a>>) {
        let lib = LispLibrary::find(_lib);
        // Then, add new module into it.
    }
}
