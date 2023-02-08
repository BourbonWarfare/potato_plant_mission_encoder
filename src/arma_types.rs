/*
    potato_plant_mission_encoder: Encodes mission data from game and compiles it into parsable history format
    Copyright (C) 2022  Bailey Danyluk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum VariableType {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<VariableType>)
}

#[derive(Error, Debug, PartialEq)]
pub enum VariableParseError {
    #[error("No data to parse from string")]
    NothingToParse,
    #[error("Data is assumed to be an array, no closing bracket found")]
    ArrayNoClosingBracket,
    #[error("Data is assumed to be a string, no closing quote found")]
    StringNoClosingQuote,
    #[error("Data is assumed to be a number, found non numeric characters")]
    NumberBadData
}

impl VariableType {
    pub fn get_type_from_string(data: &str) -> Result<VariableType, VariableParseError> {
        if data.is_empty() {
            return Err(VariableParseError::NothingToParse) 
        }

        if data == "true" || data == "false" {
            return Ok(VariableType::Boolean(data == "true"))
        }

        if data.chars().next().unwrap() == '[' {
            if data.chars().last().unwrap() != ']' {
                return Err(VariableParseError::ArrayNoClosingBracket)
            }

            let stripped_str = data.strip_prefix("[").unwrap();

            enum ParseState {
                Begin,
                Number,
                Boolean,
                Array,
                String
            }

            let mut state = ParseState::Begin;
            let mut array_types = Vec::new();

            let mut working_type = Vec::new();
            let mut bracket_counter = 0;

            // Change program to write variables to stack of
            // variables to fix array problem
            let mut end_type_parse = |working_type: &mut Vec<char>| -> Result<(), VariableParseError> {
                if !working_type.is_empty() {
                    let type_str: String = working_type.iter().collect();
                    let variable = VariableType::get_type_from_string(&type_str)?;
                    array_types.push(variable);
                    working_type.clear();
                }
                Ok(())
            };

            for c in stripped_str.chars() {
                match state {
                    ParseState::Begin => {
                        working_type.clear();
                        working_type.push(c);
                        if c.is_digit(10) {
                            state = ParseState::Number;
                        } else if c.is_alphabetic() {
                            state = ParseState::Boolean;
                        } else if c == '[' {
                            state = ParseState::Array;
                            bracket_counter = 1;
                        } else if c == '"' {
                            state = ParseState::String;
                        }
                    },
                    ParseState::Number => {
                        if c == ',' || c == ']' {
                            end_type_parse(&mut working_type)?;
                            state = ParseState::Begin;
                        }
                        working_type.push(c);
                    },
                    ParseState::Boolean => {
                        if c == ',' || c == ']' {
                            end_type_parse(&mut working_type)?;
                            state = ParseState::Begin;
                        }
                        working_type.push(c);
                    },
                    ParseState::Array => {
                        working_type.push(c);
                        if c == '[' {
                            bracket_counter += 1;
                        } else if c == ']' {
                            bracket_counter -= 1;
                        }
                        if bracket_counter == 0 {
                            state = ParseState::Begin;
                            end_type_parse(&mut working_type)?;
                        }
                    },
                    ParseState::String => {
                        working_type.push(c);
                        if c == '"' {
                            state = ParseState::Begin;
                            end_type_parse(&mut working_type)?;
                        }
                    }
                }
            }

            return Ok(VariableType::Array(array_types))
        }

        if data.chars().next().unwrap() == '"' {
            if data.chars().last().unwrap() != '"' {
                return Err(VariableParseError::StringNoClosingQuote)
            }
            return Ok(VariableType::String(data.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string()))
        }

        if let Ok(d) = data.parse::<f64>() {
            return Ok(VariableType::Number(d))
        }
        
        return Err(VariableParseError::NumberBadData)
    }
}

#[cfg(test)]
mod variable_tests {
    use super::*;

    #[test]
    fn get_type_from_string_returns_correct_type() {
        assert_eq!(VariableType::get_type_from_string(""), Err(VariableParseError::NothingToParse));
        assert_eq!(VariableType::get_type_from_string("982365.4232"), Ok(VariableType::Number(982365.4232)));
        assert_eq!(VariableType::get_type_from_string("\"foo\"abc\"true false [12]\""), Ok(VariableType::String("foo\"abc\"true false [12]".to_string())));
        assert_eq!(VariableType::get_type_from_string("true"), Ok(VariableType::Boolean(true)));
        assert_eq!(VariableType::get_type_from_string("false"), Ok(VariableType::Boolean(false)));
        assert_eq!(VariableType::get_type_from_string("[]"), Ok(VariableType::Array(vec![])));
    }

    #[test]
    fn get_type_from_string_ensure_number_converts() {
        assert_eq!(VariableType::get_type_from_string("1"), Ok(VariableType::Number(1.0)));
        assert_eq!(VariableType::get_type_from_string("1.5"), Ok(VariableType::Number(1.5)));
        assert_eq!(VariableType::get_type_from_string("-1"), Ok(VariableType::Number(-1.0)));
        assert_eq!(VariableType::get_type_from_string("-1.5"), Ok(VariableType::Number(-1.5)));
        assert_eq!(VariableType::get_type_from_string("two"), Err(VariableParseError::NumberBadData));
    }

    #[test]
    fn get_type_from_string_ensure_string_converts() {
        assert_eq!(VariableType::get_type_from_string("\"true\""), Ok(VariableType::String("true".to_string())));
        assert_eq!(VariableType::get_type_from_string("\"foobar"), Err(VariableParseError::StringNoClosingQuote));
    }

    #[test]
    fn get_type_from_string_ensure_boolean_converts() {
        assert_eq!(VariableType::get_type_from_string("true"), Ok(VariableType::Boolean(true)));
        assert_eq!(VariableType::get_type_from_string("false"), Ok(VariableType::Boolean(false)));
    }

    #[test]
    fn get_type_from_string_ensure_array_converts() {
        assert_eq!(VariableType::get_type_from_string("[1,2,3,4]"), Ok(VariableType::Array(vec![
            VariableType::Number(1.0), VariableType::Number(2.0), VariableType::Number(3.0), VariableType::Number(4.0)
        ])));
        assert_eq!(VariableType::get_type_from_string("[\"foo\", false]"), Ok(VariableType::Array(vec![
            VariableType::String("foo".to_string()), VariableType::Boolean(false)
        ])));
        assert_eq!(VariableType::get_type_from_string("[1,2,3,4"), Err(VariableParseError::ArrayNoClosingBracket));
        assert_eq!(VariableType::get_type_from_string("[foo]"), Err(VariableParseError::NumberBadData));
        assert_eq!(VariableType::get_type_from_string("[[]]"), Ok(VariableType::Array(vec![VariableType::Array(vec![])])));
        assert_eq!(VariableType::get_type_from_string("[[1, [], [2]]]"), Ok(VariableType::Array(vec![VariableType::Array(vec![VariableType::Number(1.0), VariableType::Array(vec![]), VariableType::Array(vec![VariableType::Number(2.0)])])])));
        assert_eq!(VariableType::get_type_from_string("[1, \"hello, world!\", [2, \"hi, JOHN!\", [3], [4]], [5, \"foo\"]]"), Ok(VariableType::Array(vec![
            VariableType::Number(1.0), VariableType::String("hello, world!".to_string()), VariableType::Array(vec![
                VariableType::Number(2.0), VariableType::String("hi, JOHN!".to_string()), VariableType::Array(vec![
                    VariableType::Number(3.0)
                ]), VariableType::Array(vec![
                    VariableType::Number(4.0)
                ])
            ]), VariableType::Array(vec![
                VariableType::Number(5.0), VariableType::String("foo".to_string())
            ])
        ])));
    }
}

