use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Integer(isize),
    String(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Primitive(Primitive),
    UnaryOperator((String, Box<Node>)),
    BinaryOperator((String, Box<Node>, Box<Node>)),
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Node {
        self.parse_node()
    }

    fn parse_node(&mut self) -> Node {
        match self.tokens[self.position] {
            Token::Integer(value) => {
                self.position += 1;
                Node::Primitive(Primitive::Integer(value as isize))
            }
            Token::String(ref value) => {
                self.position += 1;
                Node::Primitive(Primitive::String(value.clone()))
            }
            Token::Boolean(value) => {
                self.position += 1;
                Node::Primitive(Primitive::Boolean(value))
            }
            Token::UnaryOperator(_) => self.parse_unary(),
            Token::BinaryOperator(_) => self.parse_binary(),
            _ => panic!("Expected integer or unary operator"),
        }
    }

    fn parse_unary(&mut self) -> Node {
        let operator = match self.tokens[self.position] {
            Token::UnaryOperator(ref operator) => operator.clone(),
            _ => panic!("Expected unary operator"),
        };
        self.position += 1;
        let operand = Box::new(self.parse_node());
        Node::UnaryOperator((operator, operand))
    }

    fn parse_binary(&mut self) -> Node {
        let operator = match self.tokens[self.position] {
            Token::BinaryOperator(ref operator) => operator.clone(),
            _ => panic!("Expected binary operator"),
        };
        self.position += 1;
        let left = Box::new(self.parse_node());
        let right = Box::new(self.parse_node());
        Node::BinaryOperator((operator, left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unary() {
        let tokens = vec![Token::UnaryOperator("-".to_string()), Token::Integer(3)];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse_unary();
        assert_eq!(
            node,
            Node::UnaryOperator((
                "-".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(3)))
            ))
        );
    }

    #[test]
    fn test_parse_nested_unary() {
        let tokens = vec![
            Token::UnaryOperator("-".to_string()),
            Token::UnaryOperator("-".to_string()),
            Token::Integer(3),
        ];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse_unary();
        assert_eq!(
            node,
            Node::UnaryOperator((
                "-".to_string(),
                Box::new(Node::UnaryOperator((
                    "-".to_string(),
                    Box::new(Node::Primitive(Primitive::Integer(3)))
                )))
            ))
        );
    }

    #[test]
    fn test_parse_binary() {
        let tokens = vec![
            Token::BinaryOperator("+".to_string()),
            Token::Integer(3),
            Token::Integer(4),
        ];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse_binary();
        assert_eq!(
            node,
            Node::BinaryOperator((
                "+".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(3))),
                Box::new(Node::Primitive(Primitive::Integer(4)))
            ))
        );
    }

    #[test]
    fn test_parse_nested_binary() {
        let tokens = vec![
            Token::BinaryOperator("+".to_string()),
            Token::Integer(3),
            Token::BinaryOperator("+".to_string()),
            Token::Integer(4),
            Token::Integer(5),
        ];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse_binary();
        assert_eq!(
            node,
            Node::BinaryOperator((
                "+".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(3))),
                Box::new(Node::BinaryOperator((
                    "+".to_string(),
                    Box::new(Node::Primitive(Primitive::Integer(4))),
                    Box::new(Node::Primitive(Primitive::Integer(5)))
                )))
            ))
        );
    }
}
