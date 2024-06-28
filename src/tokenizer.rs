use std::iter::Peekable;
use std::vec::IntoIter;

pub type PeekableIter<T> = Peekable<IntoIter<T>>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(String),
    Boolean(bool),
    String(String),
    UnaryOperator(String),
    Unknown(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Integer(value) => value.to_string(),
            Token::Boolean(value) => value.to_string(),
            Token::String(value) => value.to_string(),
            Token::UnaryOperator(value) => value.to_string(),
            Token::Unknown(value) => value.to_string(),
        }
    }
}

struct Tokenizer {
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
        while let Some(&c) = self.input.peek() {
            match c {
                'I' => {
                    self.input.next();
                    continue;
                }
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::Integer(value)
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
        while let Some(&c) = self.input.peek() {
            match c {
                'S' => {
                    self.input.next();
                    continue;
                }
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::String(value)
    }

    fn tokenize_unary_operator(&mut self) -> Token {
        let mut value = String::new();
        while let Some(&c) = self.input.peek() {
            match c {
                'U' => {
                    self.input.next();
                    continue;
                }
                ' ' => break,
                _ => {
                    value.push(c);
                    self.input.next();
                }
            }
        }
        Token::UnaryOperator(value)
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
            Token::Integer("/6".to_string()),
            Token::Unknown("+".to_string()),
            Token::Integer("5".to_string()),
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
        let expected = vec![Token::String("B%,,/}Q/2,$_".to_string())];
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
}
