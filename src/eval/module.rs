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

pub struct LispModule {
    basic: LispModuleBasic<HashMap<String, super::env::LispType>>,
    modules: Vec<String>,
    tokens: super::token::LispTokens
}

pub struct LispLibrary {
    basic: LispModuleBasic<Vec<LispModule>>
}

impl LispLibrary {
    /// New for LISP_MODULES.
    pub fn new(_name: &str, def: &str) {
        unsafe {
            LISP_MODULES.push(LispLibrary{
                basic: LispModuleBasic{
                    name: _name.to_string(),
                    definition: def.to_string(),
                    private: Vec::new(),
                    public: Vec::new()
                }
            });
        }
    }

    // The function for getting specific library & modify it
    pub fn get(lib: &'static str) -> Option<&mut LispLibrary> {
        let mut result: Option<&mut LispLibrary> = None;
        unsafe {
            for e in LISP_MODULES.iter_mut() {
                if &e.basic.name == lib {
                    result = Some(e)
                }
            }
        }
        result
    }

    pub fn find(lib: &'static str) -> Option<&LispLibrary> {
        let mut result: Option<&LispLibrary> = None;
        unsafe {
            for e in LISP_MODULES.iter() {
                if &e.basic.name == lib {
                    result = Some(&e)
                }
            }
        }
        result
    }

    pub fn get_prop(&self, is_name: bool) -> &str {
        if is_name {
            &self.basic.name
        } else {
            &self.basic.definition
        }
    }
}

impl LispModule {
    pub fn new(_lib: &'static str, _name: &str, def: &str, is_private: bool,
               tokens: Vec<super::token::LispToken> ) -> Result<(), String> {
        let lib = LispLibrary::get(_lib);
        match lib {
            None => Err("The library is not exists!".to_string()),
            Some(paren_lib) => {
                let new_module = LispModule {
                    basic: LispModuleBasic {
                        name: _name.to_string(),
                        definition: def.to_string(),
                        public: HashMap::new(),
                        private: HashMap::new()
                    },
                    modules: Vec::new(),
                    tokens: super::token::LispTokens::Tokens(tokens)
                    // Debug
                    // tokens: super::token::LispTokens::EmptyToken
                };

                if is_private {
                    paren_lib.basic.private.push(new_module);
                } else {
                    paren_lib.basic.public.push(new_module);
                }
                Ok(())
            }
        }
    }

    pub fn get_prop(&self, is_name: bool) -> &str {
        if is_name {
            &self.basic.name
        } else {
            &self.basic.definition
        }
    }

    // Tokens
    /// Change LispToken to EmptyToken. This function should be used after the tokens of a file have been executed.
    /// `name` is the module's name, like "std::main"
    pub fn clear_token(name: &str) {
    }

    pub fn get_token(&self) -> &super::token::LispTokens {
        &self.tokens
    }

    // Modules
    pub fn get_modules(&self) -> &Vec<String> {
        &self.modules
    }
}

/// The macro is used for convert module string like: "std::main" into Vector includes lib, module & other things.
/// Example
/// "std::a::c" => ["std", "a", "c"]
pub fn module_split_string(module_string: &String) -> Vec<&str> {
    module_string.split("::").collect()
}
