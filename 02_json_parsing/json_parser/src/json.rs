//! Manual JSON parser

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum JsonError {
    /// Received an invalid start of string character
    InvalidStartOfString(usize, char),

    /// Received an invalid start of number
    InvalidStartOfNumber(usize, char),

    /// Received an invalid start of array
    InvalidStartOfArray(usize, char),

    /// Received an invalid start of a new JSON value
    InvalidStartOfValue(usize, char),

    /// Received an invalid start of object
    InvalidStartOfObject(usize, char),

    /// Unable to parse a number at the given position
    NoNumberFound(usize),

    /// End of file found while parsing
    EndOfFileFound,

    /// Overflowed the position
    PositionOverflow(usize),

    /// Found an unclosed string
    UnclosedString(usize),

    /// Error while parsing the float
    ParseFloat(usize, std::num::ParseFloatError),

    /// Not enough data found for parsing a bool
    InvalidDataForBool(usize),

    /// Attempted to get an invalid JSON type from value
    InvalidTypeConversion,
}

/// The available JSON value types
#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum JsonValue {
    /// A number value (always f64)
    Number(f64),

    /// A String value
    String(String),

    /// A boolean value
    Boolean(bool),

    /// An array of [`JsonValue`]
    Array(Vec<JsonValue>),

    /// A map from a key to another [`JsonValue`]
    Map(HashMap<String, JsonValue>),
}

/// Macro used to implement accessor functions on [`JsonValue`]
/// that error when the type is not the requested type
macro_rules! impl_as_type {
    ($name:ident, $ty:ident, $res:ty) => {
        #[allow(dead_code)]
        pub fn $name(&'a self) -> Result<&'a $res> {
            match self {
                JsonValue::$ty(res) => Ok(res),
                _ => Err(JsonError::InvalidTypeConversion),
            }
        }
    };
}

impl<'a> JsonValue {
    impl_as_type!(as_vec, Array, Vec<JsonValue>);
    impl_as_type!(as_num, Number, f64);
    impl_as_type!(as_str, String, String);
    impl_as_type!(as_bool, Boolean, bool);
    impl_as_type!(as_map, Map, HashMap<String, JsonValue>);
}

type Result<T> = std::result::Result<T, JsonError>;

struct ParseInput<'a> {
    /// The current position in the input to read
    position: usize,

    /// The data being parsed
    data: &'a [u8],
}

impl<'a> ParseInput<'a> {
    /// Create a [`ParseInput`] from the given data
    pub fn from_str(data: &'a str) -> Self {
        Self {
            position: 0,
            data: data.as_bytes(),
        }
    }

    /// Get the length of the data being parsed
    pub fn len(&self) -> usize {
        self.data[self.position..].len()
    }

    /// Look at the next character from the current position of the input
    /// without adjusting the position
    pub fn peek(&self) -> Option<char> {
        self.data.get(self.position).map(|x| *x as char)
    }

    /// Get the next character from the current position of the input
    /// and adjust the position to the next character
    pub fn next(&mut self) -> Option<char> {
        let res = self.data.get(self.position).map(|x| *x as char);

        // Adjust the position if a valid character was found
        if res.is_some() {
            self.position += 1;
        }

        // Return the character
        res
    }

    /// Move the position forward past all whitespace and newlines
    pub fn consume_all_whitespace(&mut self) {
        let start_pos = self.position;

        for (curr_pos, next_byte) in self.data[start_pos..].iter().enumerate() {
            if !matches!(*next_byte, b' ' | b'\n') {
                self.position = start_pos + curr_pos;
                break;
            }
        }
    }

    // Set the position to the given position and then consume all whitespace
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
        self.consume_all_whitespace();
    }

    /// Increment the position by the given offset
    pub fn increment_position(&mut self, offset: usize) -> Result<()> {
        // Check for overflow of the position
        let Some(new_pos) = self.position.checked_add(offset) else {
            return Err(JsonError::PositionOverflow(self.position));
        };

        // Update the position
        self.set_position(new_pos);

        // Successful increment
        Ok(())
    }
}

/// Parse the given data string and return the parsed JSON object
pub fn parse(data: &str) -> Result<HashMap<String, JsonValue>> {
    let mut input = ParseInput::from_str(data);
    parse_map(&mut input)
}

/// Parse a string from the given data
fn parse_string(data: &mut ParseInput) -> Result<String> {
    // Ensure the first byte is a starting string quote
    let Some(next_byte) = data.next() else {
        return Err(JsonError::EndOfFileFound);
    };

    // Ensure the first byte of the string is the string delimiter
    if next_byte != '"' {
        return Err(JsonError::InvalidStartOfString(data.position, next_byte));
    }

    let start_pos = data.position;

    for (curr_pos, next_byte) in data.data[start_pos..].iter().enumerate() {
        // Break out once an ending string is found
        if *next_byte == b'"' {
            let result =
                String::from_utf8_lossy(&data.data[start_pos..start_pos + curr_pos]).to_string();

            // Update the position of the input
            data.position = start_pos + curr_pos + 1;

            // Eat all whitespace
            data.consume_all_whitespace();

            return Ok(result);
        }
    }

    Err(JsonError::UnclosedString(start_pos.saturating_sub(1)))
}

/// Parse a json object from the given data
fn parse_map(data: &mut ParseInput) -> Result<HashMap<String, JsonValue>> {
    // Ensure the first byte is a starting string quote
    let Some(next_byte) = data.next() else {
        return Err(JsonError::EndOfFileFound);
    };

    // Ensure the first byte is the start of a Map
    if next_byte != '{' {
        return Err(JsonError::InvalidStartOfObject(data.position, next_byte));
    }

    let mut result = HashMap::new();

    'end_map: loop {
        let Some(next_byte) = data.peek() else {
            return Err(JsonError::EndOfFileFound);
        };

        if matches!(next_byte, '}') {
            data.increment_position(1)?;
            break;
        }

        // Skip past the middle of the key/value pair
        loop {
            let Some(next_byte) = data.peek() else {
                return Err(JsonError::EndOfFileFound);
            };

            match next_byte {
                ' ' | ',' | '\n' => {
                    data.increment_position(1)?;
                }
                _ => {
                    break;
                }
            }
        }

        // Parse the key for this map
        let key = parse_string(data)?;

        // Skip past the middle of the key/value pair
        loop {
            let Some(next_byte) = data.peek() else {
                return Err(JsonError::EndOfFileFound);
            };

            match next_byte {
                ' ' | '\n' => {
                    data.increment_position(1)?;
                }
                ':' => {
                    // Skip over a comma or space
                    data.increment_position(1)?;
                    break;
                }
                '}' => {
                    // Found end of the map
                    data.increment_position(1)?;
                    break 'end_map;
                }
                _ => {
                    break;
                }
            }
        }

        // Parse the value for this key and add it to the map
        let new_value = parse_value(data)?;
        result.insert(key, new_value);
    }

    Ok(result)
}

fn parse_value(data: &mut ParseInput) -> Result<JsonValue> {
    let Some(next_byte) = data.peek() else {
        return Err(JsonError::EndOfFileFound);
    };

    Ok(match next_byte {
        '"' => JsonValue::String(parse_string(data)?),
        '0'..='9' | '-' => JsonValue::Number(parse_number(data)?),
        't' | 'f' => JsonValue::Boolean(parse_bool(data)?),
        '[' => JsonValue::Array(parse_array(data)?),
        '{' => JsonValue::Map(parse_map(data)?),
        _ => return Err(JsonError::InvalidStartOfValue(data.position, next_byte)),
    })
}

/// Parse a string from the given data
fn parse_array(data: &mut ParseInput) -> Result<Vec<JsonValue>> {
    // Ensure the first byte is a starting string quote
    let Some(next_byte) = data.next() else {
        return Err(JsonError::EndOfFileFound);
    };

    // Ensure the first byte is the start of a Vec
    if next_byte != '[' {
        return Err(JsonError::InvalidStartOfArray(data.position, next_byte));
    }

    let mut result = Vec::new();

    loop {
        let Some(next_byte) = data.peek() else {
            return Err(JsonError::EndOfFileFound);
        };

        match next_byte {
            ',' | ' ' | '\n' => {
                // Skip over a comma or space
                data.increment_position(1)?;
                continue;
            }
            ']' => {
                // Found end of the vector
                data.increment_position(1)?;
                break;
            }
            _ => {
                // No other specific array characters
            }
        }

        // Parse the next value for the array
        let new_value = parse_value(data)?;
        result.push(new_value);
    }

    Ok(result)
}

/// Parse a number from the given data
fn parse_number(data: &mut ParseInput) -> Result<f64> {
    let start_pos = data.position;

    // Check if we're at the end of the file
    let Some(next_byte) = data.peek() else {
        return Err(JsonError::EndOfFileFound);
    };

    // Ensure the first byte of the is a digit or decimal point
    if !matches!(next_byte, '0'..='9' | '.' | '-') {
        return Err(JsonError::InvalidStartOfNumber(start_pos, next_byte));
    }

    // Search forward
    for (curr_pos, next_byte) in data.data[start_pos..].iter().enumerate() {
        // Break out once an ending string is found
        if !matches!(next_byte, b'0'..=b'9' | b'.' | b'-' | b'e') {
            let result = String::from_utf8_lossy(&data.data[start_pos..start_pos + curr_pos])
                .parse::<f64>()
                .map_err(|x| JsonError::ParseFloat(start_pos, x))?;

            // Update the position based on the found number
            data.position = start_pos + curr_pos;

            // Eat all whitespace
            data.consume_all_whitespace();

            return Ok(result);
        }
    }

    // Did not find the end of the number
    Err(JsonError::NoNumberFound(start_pos))
}

/// Parse a bool from the current data
fn parse_bool(data: &mut ParseInput) -> Result<bool> {
    let start_pos = data.position;

    // Ensure we have enough data to parse a 'true' or 'false' for bool
    if data.len() < 5 {
        return Err(JsonError::InvalidDataForBool(start_pos));
    }

    if &data.data[data.position..data.position + 4] == b"true" {
        data.increment_position(4)?;
        return Ok(true);
    }

    if &data.data[data.position..data.position + 5] == b"false" {
        data.increment_position(5)?;
        return Ok(false);
    }

    Err(JsonError::InvalidDataForBool(start_pos))
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::json::{JsonError, JsonValue, ParseInput};
    use std::collections::HashMap;

    #[test]
    fn test_parse_string() {
        let mut data = ParseInput::from_str(r#""key""#);
        let res = json::parse_string(&mut data);
        let expected = "key".to_string();
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn test_parse_string_error() {
        let mut data = ParseInput::from_str(r#""key"#);
        let res = json::parse_string(&mut data);
        let expected = Err(JsonError::UnclosedString(0));
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_number1() {
        let mut data = ParseInput::from_str(r#"123,"#);
        let res = json::parse_number(&mut data);
        let expected = Ok(123.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_number2() {
        let mut data = ParseInput::from_str(r#"123.456,"#);
        let res = json::parse_number(&mut data);
        let expected = Ok(123.456);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_number3() {
        let mut data = ParseInput::from_str(r#"123,456,"#);
        let res = json::parse_number(&mut data);
        let expected = Ok(123.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_number_error() {
        let mut data = ParseInput::from_str(r#"123.456"#);
        let res = json::parse_number(&mut data);
        let expected = Err(JsonError::NoNumberFound(0));
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_bool1() {
        let mut data = ParseInput::from_str(r#"true,"#);
        let res = json::parse_bool(&mut data);
        let expected = Ok(true);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_bool2() {
        let mut data = ParseInput::from_str(r#"false"#);
        let res = json::parse_bool(&mut data);
        let expected = Ok(false);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_vec1() {
        let mut data = ParseInput::from_str(r#"[123, "hi", true]"#);
        let res = json::parse_array(&mut data);
        let expected = Ok(vec![
            JsonValue::Number(123.0),
            JsonValue::String("hi".to_string()),
            JsonValue::Boolean(true),
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_vec2() {
        let mut data = ParseInput::from_str(r#"[123, ["hi", 333], true]"#);
        let res = json::parse_array(&mut data);
        let expected = Ok(vec![
            JsonValue::Number(123.0),
            JsonValue::Array(vec![
                JsonValue::String("hi".to_string()),
                JsonValue::Number(333.0),
            ]),
            JsonValue::Boolean(true),
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_map1() {
        let mut data = ParseInput::from_str(r#"{"key1": 111, "key2": 222}"#);
        let res = json::parse_map(&mut data);
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), JsonValue::Number(111.0));
        expected.insert("key2".to_string(), JsonValue::Number(222.0));
        assert_eq!(res, Ok(expected));
    }
}
