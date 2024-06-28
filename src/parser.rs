use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Integer(isize),
    String(String),
    Boolean(bool),
    Variable(usize),
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Primitive(Primitive),
    UnaryOperator(String, Box<Node>),
    BinaryOperator(String, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Lambda(Box<Node>),
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
            Token::Variable(value) => {
                self.position += 1;
                Node::Primitive(Primitive::Variable(value))
            }
            Token::UnaryOperator(_) => self.parse_unary(),
            Token::BinaryOperator(_) => self.parse_binary(),
            Token::If => self.parse_if(),
            Token::Lambda(_) => self.parse_lambda(),
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
        Node::UnaryOperator(operator, operand)
    }

    fn parse_binary(&mut self) -> Node {
        let operator = match self.tokens[self.position] {
            Token::BinaryOperator(ref operator) => operator.clone(),
            _ => panic!("Expected binary operator"),
        };
        self.position += 1;
        let left = Box::new(self.parse_node());
        let right = Box::new(self.parse_node());
        Node::BinaryOperator(operator, left, right)
    }

    fn parse_if(&mut self) -> Node {
        self.position += 1;
        let condition = Box::new(self.parse_node());
        let then_branch = Box::new(self.parse_node());
        let else_branch = Box::new(self.parse_node());
        Node::If(condition, then_branch, else_branch)
    }

    fn parse_lambda(&mut self) -> Node {
        self.position += 1;
        let node = Box::new(self.parse_node());
        Node::Lambda(node)
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
            Node::UnaryOperator(
                "-".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(3)))
            )
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
            Node::UnaryOperator(
                "-".to_string(),
                Box::new(Node::UnaryOperator(
                    "-".to_string(),
                    Box::new(Node::Primitive(Primitive::Integer(3)))
                ))
            )
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
            Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(3))),
                Box::new(Node::Primitive(Primitive::Integer(4)))
            )
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
            Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(3))),
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Primitive(Primitive::Integer(4))),
                    Box::new(Node::Primitive(Primitive::Integer(5)))
                ))
            )
        );
    }

    #[test]
    fn test_parse_if() {
        let tokens = vec![
            Token::If,
            Token::BinaryOperator(">".to_string()),
            Token::Integer(2),
            Token::Integer(3),
            Token::String("yes".to_string()),
            Token::String("no".to_string()),
        ];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse();
        assert_eq!(
            node,
            Node::If(
                Box::new(Node::BinaryOperator(
                    ">".to_string(),
                    Box::new(Node::Primitive(Primitive::Integer(2))),
                    Box::new(Node::Primitive(Primitive::Integer(3)))
                )),
                Box::new(Node::Primitive(Primitive::String("yes".to_string()))),
                Box::new(Node::Primitive(Primitive::String("no".to_string())))
            )
        );
    }

    #[test]
    fn test_parse_lambda() {
        let tokens = vec![
            Token::Lambda(1),
            Token::BinaryOperator("+".to_string()),
            Token::Integer(1),
            Token::Integer(2),
        ];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse();
        assert_eq!(
            node,
            Node::Lambda(Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Primitive(Primitive::Integer(1))),
                Box::new(Node::Primitive(Primitive::Integer(2)))
            )))
        );

        let tokens = vec![
            Token::BinaryOperator("$".to_string()),
            Token::BinaryOperator("$".to_string()),
            Token::Lambda(2),
            Token::Lambda(3),
            Token::Variable(2),
            Token::BinaryOperator(".".to_string()),
            Token::String("Hello".to_string()),
            Token::String(" World!".to_string()),
            Token::Integer(42),
        ];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse();
        assert_eq!(
            node,
            Node::BinaryOperator(
                "$".to_string(),
                Box::new(Node::BinaryOperator(
                    "$".to_string(),
                    Box::new(Node::Lambda(Box::new(Node::Lambda(Box::new(
                        Node::Primitive(Primitive::Variable(2))
                    ))))),
                    Box::new(Node::BinaryOperator(
                        ".".to_string(),
                        Box::new(Node::Primitive(Primitive::String("Hello".to_string()))),
                        Box::new(Node::Primitive(Primitive::String(" World!".to_string())))
                    ))
                )),
                Box::new(Node::Primitive(Primitive::Integer(42)))
            )
        );
    }
}
