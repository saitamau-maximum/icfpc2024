use std::iter::Peekable;
use std::vec::IntoIter;

pub type PeekableIter<T> = Peekable<IntoIter<T>>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(String),
    Unknown(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Integer(value) => value.to_string(),
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
    fn test_tokenize() {
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
}
