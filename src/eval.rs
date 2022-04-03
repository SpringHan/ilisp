// The file for the things about eval.

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

pub mod token;
pub mod env;
pub mod module;

// The mod for parser
pub mod parse {
    use super::token;

    pub fn parse_file(file_content: String) {
    }

    /// The function to check if `token` is a legal name for Lisp variable.
    pub fn is_legal_name(token: &String) -> bool {
        for c in token.chars() {
            match c {
                '-' | '0'..='9' | 'a'..='z' | 'A'..='Z' => (),
                _ => return false
            }
        }

        true
    }

    /// Get the type of `token`.
    fn get_token_type(token: &String) -> token::ParseTokenType {
        if token.starts_with("\"") {
            token::ParseTokenType::String
        } else {
            let mut has_number = false;
            let mut has_other_thing = false;

            for c in token.chars() {
                match c {
                    '0'..='9' => (),
                    _ => { return token::ParseTokenType::Symbol }
                }
            }

            token::ParseTokenType::Number
        }
    }

    // The function for calculating the line number of syntax error.
    fn calc_current_line(read_result: &Vec<String>) -> u16 {
        let mut line = 1;
        for ele in read_result.iter() {
            if ele == "\n" {
                line += 1;
            }
        }

        line
    }

    /// The function to report syntax error when meet a syntax error.
    /// `message`: String is the message comes from `LispTokens::Error`
    /// `line`: u16 is the line from `LispTokens::Error`
    /// `file`: String is the file where error comes from
    pub fn report_syntax_error(message: String, line: u16, file: String) {
        if line == 0 {
            println!("Syntax Error: {}", message);
        } else {
            println!("Syntax Error: {}  at {}, in {}", message, line, file);
        }
    }

    /// Build token. Used to traveral the `result_list` convert String token to LispToken.
    /// Then return the complete token & current line number.
    // TODO: Finish token building.
    fn build_token(result_list: &Vec<String>, _line: u16) -> (token::LispToken, u16) {
        // let mut focusing_a_token = false; // Whether now is editing a certain token.
        let mut result_token: token::LispToken;
        let mut new_token_type = 0; // Current token type.
        let mut is_empty_result = true;
        let mut index: usize = 0;
        let mut line = _line;
        let length = result_list.len();

        while index <= length {
            let token = result_list.get(index).unwrap();
            match token {
                start if start == "(" || start == "[" => {
                    if is_empty_result {
                        is_empty_result = false;
                        new_token_type = if start == "(" { 3 } else { 4 };
                        result_token = token::LispToken::new(new_token_type - 1, line);
                        index += 1;
                    } else {
                        let child_token = build_token(result_list.get(index..).unwrap(), line);
                    }
                },

                _ => {
                }
            }
        }
    }

    /// The parse function for parse the code in String to LispTokens.
    pub fn parse<'a>(code: String) -> Result<token::LispTokens<'a>, (String, u16)> {
        // First, convert code into String vector.
        let special_pair_p = |string: &str| -> bool {
            match string {
                "(" | ")" | "[" | "]" => true,
                _ => false
            }
        };
        let mut read_result: Vec<String> = Vec::new();
        let mut current_reading: String = String::new();
        for c in code.split("") {
            match c {
                "\n" => {
                    read_result.push("\n".to_string());
                    if current_reading == ";" || !current_reading.starts_with("\"") {
                        current_reading = "".to_string();
                    }
                },

                ";" => {
                    if current_reading.starts_with("\"") {
                        current_reading.push_str(";");
                    } else {
                        if !current_reading.is_empty() {
                            read_result.push(current_reading);
                        }
                        current_reading = ";".to_string();
                    }
                },

                "\"" => {
                    if !current_reading.is_empty() {
                        current_reading.push_str("\"");
                        read_result.push(current_reading);
                        current_reading = "".to_string();
                    } else {
                        current_reading = "\"".to_string();
                    }
                },

                // For the contactor like '(', ')', '[', ']'
                special if special_pair_p(special) => {
                    if !current_reading.is_empty() {
                        if current_reading.starts_with("\"") {
                            current_reading.push_str(special.to_string());
                        } else {
                            read_result.push(current_reading);
                            current_reading = "".to_string();
                        }
                    } else {
                        read_result.push(special.to_string());
                    }
                },

                " " => {
                    if current_reading.starts_with("\"") {
                        current_reading.push_str(" ");
                    } else {
                        read_result.push(current_reading);
                        current_reading = "".to_string();
                    }
                },

                others => {
                    // if !current_reading.is_empty() &&
                    //     !current_reading.starts_with("\"") &&
                    //     !is_legal_name(&current_reading) &&
                    //     is_legal_name(&others.to_string()) { // The conditions for get prefix function
                    //         current_reading.insert_str(0, "^");
                    //         read_result.push(current_reading);
                    //         current_reading = "".to_string();
                    //     }
                    current_reading.push_str(others);
                }
            }
        }

        if !current_reading.is_empty() {
        }

        if read_result.is_empty() {
            return Ok(token::LispTokens::EmptyToken)
        }

        // println!("{:?}", read_result);

        // Secondly, Build up token
        let mut token_result: Vec<token::LispToken> = Vec::new();
        let mut current_line = 1;
        for i in 0..read_result.len() {
            let new_token_result = build_token(read_result.get(i..).unwrap(), current_line);
            let new_token = new_token_result.0;
            current_line = new_token_result.1; // Update line number
            token_result.push(new_token);
        }

        // Ok(token::LispTokens::Tokens(token_result))

        // Debug
        Ok(token::LispTokens::EmptyToken)
    }
}
