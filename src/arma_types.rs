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

#[derive(Debug, PartialEq)]
pub enum VariableType {
    Unknown,
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<VariableType>)
}

impl VariableType {
    pub fn get_type_from_string(data: &str) -> VariableType {
        if data.is_empty() {
            return VariableType::Unknown  
        }

        if data == "true" || data == "false" {
            return VariableType::Boolean(data == "true") 
        }

        if data.chars().next().unwrap() == '[' {
            if data.chars().last().unwrap() != ']' {
                return VariableType::Unknown 
            }

            let mut array_items_str = Vec::new();
            data.strip_prefix("[")
                .unwrap().strip_suffix("]")
                .unwrap().split(",")
                .for_each(|item| {
                    if item != "" {
                        array_items_str.push(item);
                    }
                });

            let mut array_types = Vec::new();
            for array_item in array_items_str {
                array_types.push(VariableType::get_type_from_string(array_item));
            }

            return VariableType::Array(array_types) 
        }

        if data.chars().next().unwrap() == '"' {
            if data.chars().last().unwrap() != '"' {
                return VariableType::Unknown 
            }
            return VariableType::String(data.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string()) 
        }

        if let Ok(d) = data.parse::<f64>() {
            VariableType::Number(d)
        } else {
            VariableType::Unknown 
        }
    }
}

#[cfg(test)]
mod variable_tests {
    use super::*;

    #[test]
    fn get_type_from_string_returns_correct_type() {
        assert_eq!(VariableType::get_type_from_string(""), VariableType::Unknown);
        assert_eq!(VariableType::get_type_from_string("982365.4232"), VariableType::Number(982365.4232));
        assert_eq!(VariableType::get_type_from_string("\"foo\"abc\"true false [12]\""), VariableType::String("foo\"abc\"true false [12]".to_string()));
        assert_eq!(VariableType::get_type_from_string("true"), VariableType::Boolean(true));
        assert_eq!(VariableType::get_type_from_string("false"), VariableType::Boolean(false));
        assert_eq!(VariableType::get_type_from_string("[]"), VariableType::Array(vec![]));
    }

    #[test]
    fn get_type_from_string_ensure_number_converts() {
        assert_eq!(VariableType::get_type_from_string("1"), VariableType::Number(1.0));
        assert_eq!(VariableType::get_type_from_string("1.5"), VariableType::Number(1.5));
        assert_eq!(VariableType::get_type_from_string("-1"), VariableType::Number(-1.0));
        assert_eq!(VariableType::get_type_from_string("-1.5"), VariableType::Number(-1.5));
        assert_eq!(VariableType::get_type_from_string("two"), VariableType::Unknown);
    }

    #[test]
    fn get_type_from_string_ensure_string_converts() {
        assert_eq!(VariableType::get_type_from_string("\"true\""), VariableType::String("true".to_string()));
        assert_eq!(VariableType::get_type_from_string("\"foobar"), VariableType::Unknown);
    }

    #[test]
    fn get_type_from_string_ensure_boolean_converts() {
        assert_eq!(VariableType::get_type_from_string("true"), VariableType::Boolean(true));
        assert_eq!(VariableType::get_type_from_string("false"), VariableType::Boolean(false));
    }

    #[test]
    fn get_type_from_string_ensure_array_converts() {
        assert_eq!(VariableType::get_type_from_string("[1,2,3,4]"), VariableType::Array(vec![
            VariableType::Number(1.0), VariableType::Number(2.0), VariableType::Number(3.0), VariableType::Number(4.0)
        ]));
        assert_eq!(VariableType::get_type_from_string("[1,2,[3,4]]"), VariableType::Array(vec![
            VariableType::Number(1.0), VariableType::Number(2.0), VariableType::Array(vec![
                VariableType::Number(3.0), VariableType::Number(4.0)
            ])
        ]));
        assert_eq!(VariableType::get_type_from_string("[\"foo\", false]"), VariableType::Array(vec![
            VariableType::String("foo".to_string()), VariableType::Boolean(false)
        ]));

        assert_eq!(VariableType::get_type_from_string("[1,2,3,4"), VariableType::Unknown);
        assert_eq!(VariableType::get_type_from_string("[foo]"), VariableType::Array(vec![VariableType::Unknown]));
    }
}
