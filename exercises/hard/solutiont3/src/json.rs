use std::collections::HashMap;
use std::iter::Peekable;
use std::num::{ParseFloatError, ParseIntError};
use std::vec;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    String(String),
    Number(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, position: 0 }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let current_char = self.peek_char();
            if current_char.is_whitespace() {
                self.consume_whitespace();
            } else if current_char == '"' {
                tokens.push(self.consume_string()?);
            } else if current_char.is_digit(10) || current_char == '-' {
                tokens.push(self.consume_number()?);
            } else {
                match current_char {
                    '{' => {
                        self.position += 1;
                        tokens.push(Token::LeftBrace);
                    }
                    '}' => {
                        self.position += 1;
                        tokens.push(Token::RightBrace);
                    }
                    '[' => {
                        self.position += 1;
                        tokens.push(Token::LeftBracket);
                    }
                    ']' => {
                        self.position += 1;
                        tokens.push(Token::RightBracket);
                    }
                    ',' => {
                        self.position += 1;
                        tokens.push(Token::Comma);
                    }
                    ':' => {
                        self.position += 1;
                        tokens.push(Token::Colon);
                    }
                    't' => {
                        tokens.push(self.consume_keyword("true", Token::True)?);
                    }
                    'f' => {
                        tokens.push(self.consume_keyword("false", Token::False)?);
                    }
                    'n' => {
                        tokens.push(self.consume_keyword("null", Token::Null)?);
                    }
                    invalid_char => {
                        return Err(format!("Unexpected character: {}", invalid_char));
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn consume_whitespace(&mut self) {
        while self.position < self.input.len() && self.peek_char().is_whitespace() {
            self.position += 1;
        }
    }

    fn peek_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn consume_char(&mut self) -> char {
        let current_char = self.peek_char();
        self.position += current_char.len_utf8();
        current_char
    }

    fn consume_string(&mut self) -> Result<Token, String> {
        self.consume_char(); // Skip the opening quote (")
        let mut string_value = String::new();

        while self.position < self.input.len() {
            let current_char = self.consume_char();
            if current_char == '"' {
                return Ok(Token::String(string_value));
            } else if current_char == '\\' {
                // Handle escape sequences (e.g., \", \n, \\)
                let escape_char = self.consume_char();
                match escape_char {
                    '"' | '\\' | '/' => string_value.push(escape_char),
                    'b' => string_value.push('\u{0008}'),
                    'f' => string_value.push('\u{000C}'),
                    'n' => string_value.push('\n'),
                    'r' => string_value.push('\r'),
                    't' => string_value.push('\t'),
                    _ => return Err(format!("Invalid escape character: {}", escape_char)),
                }
            } else {
                string_value.push(current_char);
            }
        }
        Err("Unterminated string".to_string())
    }

    fn consume_number(&mut self) -> Result<Token, String> {
        let start = self.position;

        // Handle negative numbers
        if self.peek_char() == '-' {
            self.position += 1;
        }

        while self.position < self.input.len() && self.peek_char().is_digit(10) {
            self.position += 1;
        }

        // Handle the decimal part
        if self.position < self.input.len() && self.peek_char() == '.' {
            self.position += 1;
            while self.position < self.input.len() && self.peek_char().is_digit(10) {
                self.position += 1;
            }
        }

        // Handle exponent part (e.g., 1.23e4)
        if self.position < self.input.len() && (self.peek_char() == 'e' || self.peek_char() == 'E')
        {
            self.position += 1;
            if self.position < self.input.len()
                && (self.peek_char() == '-' || self.peek_char() == '+')
            {
                self.position += 1;
            }
            while self.position < self.input.len() && self.peek_char().is_digit(10) {
                self.position += 1;
            }
        }

        let number_str = &self.input[start..self.position];
        Ok(Token::Number(number_str.to_string()))
    }

    fn consume_keyword(&mut self, keyword: &str, token: Token) -> Result<Token, String> {
        for c in keyword.chars() {
            if self.position < self.input.len() && self.consume_char() == c {
                continue;
            } else {
                return Err(format!("Expected keyword '{}'", keyword));
            }
        }
        Ok(token)
    }
}

struct TokenIterator {
    tokens: Peekable<vec::IntoIter<Token>>,
    buffer: Option<Token>,
}

impl TokenIterator {
    pub fn new(tokens: Vec<Token>) -> TokenIterator {
        Self {
            tokens: tokens.into_iter().peekable(),
            buffer: None,
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.buffer.as_ref()
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.buffer = self.tokens.next();
        self.current()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct JsonNumber(String);

#[derive(Debug)]
pub(crate) enum JsonObject {
    Object(HashMap<String, JsonObject>),
    Array(Vec<JsonObject>),
    String(String),
    Number(JsonNumber),
    True,
    False,
    Null,
}

impl JsonNumber {
    pub fn new(num: String) -> Self {
        Self { 0: num }
    }

    pub fn to_i32(&self) -> Result<i32, ParseIntError> {
        self.0.parse::<i32>()
    }

    pub fn to_f32(&self) -> Result<f32, ParseFloatError> {
        self.0.parse::<f32>()
    }
}

impl JsonObject {
    pub fn parse(input: &str) -> Result<JsonObject, String> {
        let tokens = Tokenizer::new(input).tokenize()?;
        let mut iterator = TokenIterator::new(tokens);
        iterator.advance(); // make current -> {
        Self::parse_object(&mut iterator)
    }

    pub fn object(&self) -> Option<&HashMap<String, JsonObject>> {
        match self {
            JsonObject::Object(value) => Some(value),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<&Vec<JsonObject>> {
        match self {
            JsonObject::Array(value) => Some(value),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<&String> {
        match self {
            JsonObject::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn number(&self) -> Option<&JsonNumber> {
        match self {
            JsonObject::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn bool(&self) -> Option<bool> {
        match self {
            JsonObject::True => Some(true),
            JsonObject::False => Some(false),
            _ => None,
        }
    }

    pub fn null(&self) -> bool {
        match self {
            JsonObject::Null => true,
            _ => false,
        }
    }

    fn parse_object(iterator: &mut TokenIterator) -> Result<JsonObject, String> {
        // Ensure the first token is '{'
        if iterator.current() != Some(&Token::LeftBrace) {
            return Err(format!("Missing object, found {:?}", iterator.current()));
        }
        iterator.advance(); // Move past '{'

        let mut root: HashMap<String, JsonObject> = HashMap::new();

        // Check for an empty object '{}'
        if iterator.current() == Some(&Token::RightBrace) {
            iterator.advance(); // Consume '}'
            return Ok(JsonObject::Object(root));
        }

        while let Some(token) = iterator.current().cloned() {
            match token {
                Token::String(key) => {
                    iterator.advance(); // Move to `:` after key
                    if iterator.current() != Some(&Token::Colon) {
                        return Err("Missing colon in the object!".to_string());
                    }
                    iterator.advance(); // Move past `:`, now on value

                    let value = Self::parse_value(iterator)?;
                    root.insert(key, value);

                    match iterator.current() {
                        Some(Token::Comma) => {
                            iterator.advance(); // Consume comma and move to the next key-value pair
                            if iterator.peek().is_none() {
                                return Err("Trailing comma in the object!".to_string());
                            }
                        }
                        Some(Token::RightBrace) => {
                            iterator.advance(); // Consume '}'
                            return Ok(JsonObject::Object(root));
                        }
                        Some(unexpected) => {
                            return Err(format!("Unexpected token {:?} in object", unexpected));
                        }
                        None => {
                            return Err("Unexpected end of input while parsing object".to_string());
                        }
                    }
                }
                Token::RightBrace => {
                    iterator.advance(); // Consume '}'
                    return Ok(JsonObject::Object(root));
                }
                unexpected => {
                    return Err(format!("Expected a string key, but found {:?}", unexpected));
                }
            }
        }

        Err("Unterminated object, missing closing '}'".to_string())
    }

    fn parse_array(mut iterator: &mut TokenIterator) -> Result<JsonObject, String> {
        let token = iterator.current();
        if !(token.is_some() && *token.unwrap() == Token::LeftBracket) {
            return Err("Missing [ at the start!".to_owned());
        }
        let mut token = iterator.advance();

        // empty array
        if token.is_some() && *token.unwrap() == Token::RightBracket {
            iterator.advance();
            return Ok(JsonObject::Array(vec![]));
        }

        let mut values = vec![];
        // non-empty
        while token.is_some() {
            values.push(Self::parse_value(&mut iterator)?);
            let next_token = iterator.current();
            if next_token.is_none() {
                return Err("Unterminated char at the end of array!".to_owned());
            }
            match next_token.unwrap() {
                Token::RightBracket => {
                    iterator.advance();
                    return Ok(JsonObject::Array(values));
                }
                Token::Comma => {
                    token = iterator.advance();
                }
                _ => {
                    return Err(format!(
                        "Illegal char in the end of [value]! {:?}",
                        next_token.unwrap()
                    ));
                }
            }
        }

        Err("Unterminated char at the end of array!".to_owned())
    }

    fn parse_value(mut iterator: &mut TokenIterator) -> Result<JsonObject, String> {
        let current = match iterator.current() {
            Some(token) => Some(token.clone()),
            None => None,
        };
        if current.is_none() {
            return Err("Error parsing value!".to_owned());
        }
        match current.unwrap() {
            Token::String(value) => {
                iterator.advance();
                Ok(JsonObject::String(value.clone()))
            } // string
            Token::Number(value) => {
                iterator.advance();
                Ok(JsonObject::Number(JsonNumber::new(value.clone())))
            } // number
            Token::LeftBrace => Self::parse_object(&mut iterator), // object
            Token::LeftBracket => Self::parse_array(&mut iterator), // array
            Token::True => {
                iterator.advance();
                Ok(JsonObject::True)
            }
            Token::False => {
                iterator.advance();
                Ok(JsonObject::False)
            }
            Token::Null => {
                iterator.advance();
                Ok(JsonObject::Null)
            }
            illegal_token => Err(format!(
                "Illegal Token in parsing value {:?}!",
                illegal_token
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    mod tokenizer {
        use super::super::*;
        #[test]
        fn test_tokenize_basic_json() {
            let input = r#"{
                "name": "Alice",
                "age": 30
            }"#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![
                Token::LeftBrace,
                Token::String("name".to_string()),
                Token::Colon,
                Token::String("Alice".to_string()),
                Token::Comma,
                Token::String("age".to_string()),
                Token::Colon,
                Token::Number(30.to_string()),
                Token::RightBrace,
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn test_tokenize_array_json() {
            let input = r#"[1, 2, 3, 4]"#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![
                Token::LeftBracket,
                Token::Number(1.to_string()),
                Token::Comma,
                Token::Number(2.to_string()),
                Token::Comma,
                Token::Number(3.to_string()),
                Token::Comma,
                Token::Number(4.to_string()),
                Token::RightBracket,
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn test_tokenize_string_with_escapes() {
            let input = r#""Hello, \"world\"!""#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![Token::String("Hello, \"world\"!".to_string())];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn test_tokenize_boolean_and_null() {
            let input = r#"true, false, null"#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![
                Token::True,
                Token::Comma,
                Token::False,
                Token::Comma,
                Token::Null,
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn test_tokenize_invalid_input() {
            let input = r#"{ "name": "Alice", "age": "30""#;

            let mut tokenizer = Tokenizer::new(input);
            let result = tokenizer.tokenize();

            assert!(result.is_ok(), "Tokenizer does not check syntax error!");
        }

        #[test]
        fn test_tokenize_number_with_exponent() {
            let input = r#"{
                "value": 3.14e10
            }"#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![
                Token::LeftBrace,
                Token::String("value".to_string()),
                Token::Colon,
                Token::Number("3.14e10".to_string()),
                Token::RightBrace,
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn test_tokenize_empty_object() {
            let input = r#"{}"#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![Token::LeftBrace, Token::RightBrace];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn test_tokenize_empty_array() {
            let input = r#"[]"#;

            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize().unwrap();

            let expected_tokens = vec![Token::LeftBracket, Token::RightBracket];

            assert_eq!(tokens, expected_tokens);
        }
    }

    mod json {
        use crate::json::*;

        #[test]
        fn test_parse_nested_object() {
            let input = r#"{ 
            "level1": { 
            "level2": { 
                "level3": { 
                    "key": "deep_value" 
                        } 
                    } 
                } 
            }"#;

            let json = JsonObject::parse(input).expect("Failed to parse JSON");
            let obj = json.object().expect("Expected an object");

            let level1 = obj
                .get("level1")
                .unwrap()
                .object()
                .expect("Expected level1 object");
            let level2 = level1
                .get("level2")
                .unwrap()
                .object()
                .expect("Expected level2 object");
            let level3 = level2
                .get("level3")
                .unwrap()
                .object()
                .expect("Expected level3 object");

            assert_eq!(level3.get("key").unwrap().string().unwrap(), "deep_value");
        }

        #[test]
        fn test_parse_mixed_object_and_array() {
            let input = r#"{ 
            "name": "John",
            "age": 30,
            "hobbies": ["reading", "cycling", "coding"],
            "address": {
                "city": "New York",
                "zip": "10001"
                }
            }"#;

            let json = JsonObject::parse(input).expect("Failed to parse JSON");
            let obj = json.object().expect("Expected an object");

            assert_eq!(obj.get("name").unwrap().string().unwrap(), "John");
            assert_eq!(
                *obj.get("age").unwrap().number().unwrap(),
                JsonNumber("30".to_string())
            );

            let hobbies = obj
                .get("hobbies")
                .unwrap()
                .array()
                .expect("Expected an array");
            assert_eq!(hobbies.len(), 3);
            assert_eq!(hobbies[0].string().unwrap(), "reading");
            assert_eq!(hobbies[1].string().unwrap(), "cycling");
            assert_eq!(hobbies[2].string().unwrap(), "coding");

            let address = obj
                .get("address")
                .unwrap()
                .object()
                .expect("Expected address object");
            assert_eq!(address.get("city").unwrap().string().unwrap(), "New York");
            assert_eq!(address.get("zip").unwrap().string().unwrap(), "10001");
        }

        #[test]
        fn test_parse_deeply_nested_json() {
            let input = r#"{ 
            "level1": { 
                "level2": { 
                    "level3": { 
                        "level4": { 
                            "level5": { 
                                "final_value": 999 
                                } 
                            } 
                        } 
                    } 
                } 
            }"#;

            let json = JsonObject::parse(input).expect("Failed to parse JSON");
            let obj = json.object().expect("Expected an object");

            let level1 = obj
                .get("level1")
                .unwrap()
                .object()
                .expect("Expected level1");
            let level2 = level1
                .get("level2")
                .unwrap()
                .object()
                .expect("Expected level2");
            let level3 = level2
                .get("level3")
                .unwrap()
                .object()
                .expect("Expected level3");
            let level4 = level3
                .get("level4")
                .unwrap()
                .object()
                .expect("Expected level4");
            let level5 = level4
                .get("level5")
                .unwrap()
                .object()
                .expect("Expected level5");

            assert_eq!(
                *level5.get("final_value").unwrap().number().unwrap(),
                JsonNumber("999".to_string())
            );
        }

        #[test]
        fn test_parse_empty_object_and_array() {
            let input = r#"{ "empty_object":{},"empty_array": [] }"#;

            let json = JsonObject::parse(input).expect("Failed to parse JSON");
            let obj = json.object().expect("Expected an object");

            assert!(obj
                .get("empty_object")
                .unwrap()
                .object()
                .unwrap()
                .is_empty());
            assert!(obj.get("empty_array").unwrap().array().unwrap().is_empty());
        }

        #[test]
        fn test_parse_nested_array() {
            let input = r#"{"test":[ [1, 2], [3, [4, 5]], [6] ]}"#;

            let json = JsonObject::parse(input).expect("Failed to parse JSON");
            let arr = json
                .object()
                .unwrap()
                .get("test")
                .unwrap()
                .array()
                .expect("Expected an array");

            assert_eq!(arr.len(), 3);

            let first = arr[0].array().expect("Expected first array");
            assert_eq!(*first[0].number().unwrap(), JsonNumber("1".to_string()));
            assert_eq!(*first[1].number().unwrap(), JsonNumber("2".to_string()));

            let second = arr[1].array().expect("Expected second array");
            assert_eq!(*second[0].number().unwrap(), JsonNumber("3".to_string()));

            let nested = second[1].array().expect("Expected nested array");
            assert_eq!(*nested[0].number().unwrap(), JsonNumber("4".to_string()));
            assert_eq!(*nested[1].number().unwrap(), JsonNumber("5".to_string()));

            let third = arr[2].array().expect("Expected third array");
            assert_eq!(*third[0].number().unwrap(), JsonNumber("6".to_string()));
        }

        #[test]
        fn test_parse_json_with_whitespace() {
            let input = r#"
            {
                "key": "value",  
                "number":  42 ,
                "boolean":    true ,
                "null": null    
            }    
            "#;

            let json = JsonObject::parse(input).expect("Failed to parse JSON");
            let obj = json.object().expect("Expected an object");

            assert_eq!(obj.get("key").unwrap().string().unwrap(), "value");
            assert_eq!(
                *obj.get("number").unwrap().number().unwrap(),
                JsonNumber("42".to_string())
            );
            assert_eq!(obj.get("boolean").unwrap().bool().unwrap(), true);
            assert!(obj.get("null").unwrap().null());
        }

        #[test]
        fn test_parse_object() {
            let input = r#"{ "key": "value", "number": 42, "boolean": true, "null": null }"#;
            let json = JsonObject::parse(input).expect("Failed to parse JSON");

            let obj = json.object().expect("Expected an object");
            assert_eq!(obj.len(), 4);

            assert_eq!(obj.get("key").unwrap().string().unwrap(), "value");
            assert_eq!(
                *obj.get("number").unwrap().number().unwrap(),
                JsonNumber("42".to_string())
            );
            assert_eq!(obj.get("boolean").unwrap().bool().unwrap(), true);
            assert!(obj.get("null").unwrap().null());
        }

        #[test]
        fn test_invalid_json_missing_bracket() {
            let input = r#"{ "key": "value", "number": 42 "#;
            assert!(JsonObject::parse(input).is_err());
        }

        #[test]
        fn test_invalid_json_unquoted_key() {
            let input = r#"{ key: "value" }"#;
            assert!(JsonObject::parse(input).is_err());
        }

        #[test]
        fn test_invalid_json_trailing_comma() {
            let input = r#"{ "key": "value", }"#;
            assert!(JsonObject::parse(input).is_err());
        }

        #[test]
        fn test_parse_array() {
            let input = r#"{"test":[ "item1", 2, false, null ]}"#;
            let json = JsonObject::parse(input).expect("Failed to parse JSON");

            let obj = json.object().expect("Expected an object");
            let arr = obj.get("test").unwrap().array().unwrap();
            assert_eq!(arr.len(), 4);

            assert_eq!(arr[0].string().unwrap(), "item1");
            assert_eq!(*arr[1].number().unwrap(), JsonNumber("2".to_string()));
            assert_eq!(arr[2].bool().unwrap(), false);
            assert!(arr[3].null());
        }

        #[test]
        fn test_invalid_json() {
            let input = "{ invalid json }";
            assert!(JsonObject::parse(input).is_err());
        }

        #[test]
        fn test_object_accessor() {
            let mut map = HashMap::new();
            map.insert("key".to_string(), JsonObject::String("value".to_string()));
            let json = JsonObject::Object(map);

            assert!(json.object().is_some());
            assert!(json.array().is_none());
        }

        #[test]
        fn test_array_accessor() {
            let json = JsonObject::Array(vec![
                JsonObject::String("item1".to_string()),
                JsonObject::Number(JsonNumber("42".to_string())),
            ]);

            assert!(json.array().is_some());
            assert!(json.object().is_none());
        }

        #[test]
        fn test_string_accessor() {
            let json = JsonObject::String("text".to_string());

            assert_eq!(json.string().unwrap(), "text");
            assert!(json.number().is_none());
        }

        #[test]
        fn test_number_accessor() {
            let json = JsonObject::Number(JsonNumber("99.9".to_string()));

            assert_eq!(*json.number().unwrap(), JsonNumber("99.9".to_string()));
            assert!(json.string().is_none());
        }

        #[test]
        fn test_bool_accessor() {
            let json_true = JsonObject::True;
            let json_false = JsonObject::False;

            assert_eq!(json_true.bool().unwrap(), true);
            assert_eq!(json_false.bool().unwrap(), false);
        }

        #[test]
        fn test_null_accessor() {
            let json = JsonObject::Null;

            assert!(json.null());
        }
    }
}
