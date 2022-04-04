// Parse file

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

use super::token;

/// The possible error types while constructuring token.
pub enum TokenError {
    MissedPair,
    WrongUse
}

impl TokenError {
    pub fn error_message(err_type: TokenError, thing: &str) -> String {
        match err_type {
            TokenError::MissedPair => format!("Missed pair: '{}'!", thing),

            TokenError::WrongUse => format!("Wrong use for '{}'!", thing)
        }
    }
}

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
/// Then return the complete token, current line number & last index of building when there's no error.
/// If there's an error during constructuring, throw Err with current line number.
fn build_token<'a>(result_list: &'a [String], _line: u16) -> Result<(token::LispToken, u16, usize), (String, u16)> {
    let mut result_token = token::LispToken::init();
    let mut is_empty_result = true;
    let mut index: usize = 0;
    let mut line = _line;
    // let mut has_new_line = false;
    let length = result_list.len();

    while index < length {
        let token = result_list.get(index).unwrap();
        match token {
            ln if ln == "\n" => {
                // if !is_empty_result {
                //     has_new_line = true;
                // }
                line += 1;
            },

            start if start == "(" || start == "[" => {
                if is_empty_result {
                    is_empty_result = false;
                    result_token = token::LispToken::new(if start == "(" { 2 } else { 3 });
                    index += 1;
                } else {
                    let child_token = match build_token(result_list.get(index..).unwrap(), line) {
                        Err(e) => return Err(e),
                        Ok(r) => r
                    };
                    line = child_token.1;
                    index = child_token.2 + 1; // Refresh the index to skip the tokens checked.
                    result_token.append_element(None, Some(child_token.0));

                    // Initialize new line
                    // if has_new_line {
                    //     has_new_line = false;
                    // }
                }
            },

            end if end == ")" || end == "]" => {
                if is_empty_result {
                    return Err((TokenError::error_message(TokenError::MissedPair, &end), line))
                }
            },

            // TODO: Convert ' & ` to quote & backquote.
            // prefix if prefix == "'" || prefix == "`" {
            // },

            other_token => {
                if is_empty_result {
                    if length != 1  {
                        return Err((TokenError::error_message(TokenError::WrongUse, &other_token), line))
                    }

                    result_token = if other_token == "()" {
                        token::LispToken::new(0)
                    } else {
                        token::LispToken::new_solo(&other_token, get_token_type(&other_token), line)
                    };
                } else {
                    if other_token == "()" {
                        result_token.append_element(None, Some(
                            token::LispToken::new(0)
                        ));
                    } else {
                        result_token.append_element(Some(
                            (other_token.to_string(), get_token_type(&other_token), line)
                        ), None);
                    }

                    // Initialize new line.
                    // if has_new_line {
                    //     has_new_line = false;
                    // }
                }

                index += 1;
            }
        }
    }

    Ok((result_token, line, index))
}

/// The parse function for parse the code in String to LispTokens.
pub fn parse(code: String) -> Result<token::LispTokens, (String, u16)> {
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
                        current_reading.push_str(special);
                    } else {
                        read_result.push(current_reading);
                        current_reading = "".to_string();
                    }
                } else {
                    if special == ")" && read_result.last().unwrap() == "(" {
                        if let Some(t) = read_result.last_mut() {
                            *t = "()".to_string();
                        }
                    } else if special == "]" && read_result.last().unwrap() == "[" {
                        return Err(("Cannot use empty Property!".to_string(), calc_current_line(&read_result)))
                    } else {
                        read_result.push(special.to_string());
                    }
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
                if current_reading.is_empty() && (others == "'" || others == "`") {
                    // Push special prefix into token vector.
                    read_result.push(others.to_string());
                } else {
                    current_reading.push_str(others);
                }
            }
        }
    }

    // If the destruction is over but the reading is not, throw an error.
    if !current_reading.is_empty() {
        return Err((TokenError::error_message(TokenError::MissedPair, ""), calc_current_line(&read_result)))
    }

    if read_result.is_empty() {
        return Ok(token::LispTokens::EmptyToken)
    }

    // Debug
    // println!("{:?}", read_result);

    // Secondly, Build up token
    let mut token_result: Vec<token::LispToken> = Vec::new();
    let mut current_line = 1;
    let mut index: usize = 0;
    while index < read_result.len() {
        let new_token_result = match build_token(read_result.get(index..).unwrap(), current_line) {
            Err(e) => return Err(e),
            Ok(r) => r
        };

        current_line = new_token_result.1;
        index = new_token_result.2;
        token_result.push(new_token_result.0);
    }

    Ok(token::LispTokens::Tokens(token_result))
}
