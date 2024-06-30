use std::iter::Peekable;
use std::vec::IntoIter;

use super::util::{convert_integer, convert_string, deconvert_integer, deconvert_string};

pub type PeekableIter<T> = Peekable<IntoIter<T>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(usize),
    Boolean(bool),
    String(String),
    UnaryOperator(String),
    BinaryOperator(String),
    If,
    Lambda(usize),
    Variable(usize),
    Unknown(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Integer(value) => format!("I{}", deconvert_integer(*value)),
            Token::Boolean(value) => {
                if *value {
                    "T".to_string()
                } else {
                    "F".to_string()
                }
            }
            Token::String(value) => format!("S{}", deconvert_string(value.to_string())),
            Token::UnaryOperator(value) => format!("U{}", value),
            Token::BinaryOperator(value) => format!("B{}", value),
            Token::If => "?".to_string(),
            Token::Lambda(value) => format!("L{}", deconvert_integer(*value)),
            Token::Variable(value) => format!("v{}", deconvert_integer(*value)),
            Token::Unknown(value) => format!("U{}", value),
        }
    }
}

pub struct Tokenizer {
    input: PeekableIter<char>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input: input.chars().collect::<Vec<char>>().into_iter().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(&c) = self.input.peek() {
            match c {
                'I' => tokens.push(self.tokenize_integer()),
                'T' | 'F' => tokens.push(self.tokenize_boolean()),
                'S' => tokens.push(self.tokenize_string()),
                'U' => tokens.push(self.tokenize_unary_operator()),
                'B' => tokens.push(self.tokenize_binary_operator()),
                '?' => tokens.push(self.tokenize_if()),
                'L' => tokens.push(self.tokenize_lambda()),
                'v' => tokens.push(self.tokenize_variable()),
                ' ' => {
                    self.input.next();
                }
                _ => tokens.push(self.tokenize_unknown()),
            }
        }
        tokens
    }

    fn tokenize_integer(&mut self) -> Token {
        let mut value = String::new();
        self.input.next();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::Integer(convert_integer(value))
    }

    fn tokenize_boolean(&mut self) -> Token {
        let mut value = String::new();
        while let Some(&c) = self.input.peek() {
            match c {
                'T' | 'F' => {
                    value.push(c);
                    self.input.next();
                }
                ' ' => break,
                _ => {
                    self.input.next();
                }
            }
        }
        Token::Boolean(value == "T")
    }

    fn tokenize_string(&mut self) -> Token {
        let mut value = String::new();
        self.input.next();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::String(convert_string(value))
    }

    fn tokenize_unary_operator(&mut self) -> Token {
        let mut value = String::new();
        self.input.next();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::UnaryOperator(value)
    }

    fn tokenize_binary_operator(&mut self) -> Token {
        let mut value = String::new();
        self.input.next();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::BinaryOperator(value)
    }

    fn tokenize_if(&mut self) -> Token {
        self.input.next();
        Token::If
    }

    fn tokenize_lambda(&mut self) -> Token {
        let mut value = String::new();
        self.input.next();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::Lambda(convert_integer(value))
    }

    fn tokenize_variable(&mut self) -> Token {
        let mut value = String::new();
        self.input.next();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::Variable(convert_integer(value))
    }

    fn tokenize_unknown(&mut self) -> Token {
        let mut value = String::new();
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::Unknown(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_integer() {
        let input = "I/6 + I5";
        let expected = vec![
            Token::Integer(1337),
            Token::Unknown("+".to_string()),
            Token::Integer(20),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_boolean() {
        let input = "T F T F";
        let expected = vec![
            Token::Boolean(true),
            Token::Boolean(false),
            Token::Boolean(true),
            Token::Boolean(false),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_string() {
        let input = "SB%,,/}Q/2,$_";
        let expected = vec![Token::String("Hello World!".to_string())];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_unary_operator() {
        let input = "U+ U- U* U/";
        let expected = vec![
            Token::UnaryOperator("+".to_string()),
            Token::UnaryOperator("-".to_string()),
            Token::UnaryOperator("*".to_string()),
            Token::UnaryOperator("/".to_string()),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_binary_operator() {
        let input = "B+ B- B* B/";
        let expected = vec![
            Token::BinaryOperator("+".to_string()),
            Token::BinaryOperator("-".to_string()),
            Token::BinaryOperator("*".to_string()),
            Token::BinaryOperator("/".to_string()),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_if() {
        let input = "?";
        let expected = vec![Token::If];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_lambda() {
        let input = "L#";
        let expected = vec![Token::Lambda(2)];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable() {
        let input = "v#";
        let expected = vec![Token::Variable(2)];
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert_eq!(result, expected);
    }
}
