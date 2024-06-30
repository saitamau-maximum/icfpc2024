use crate::icfp::util::{convert_integer, deconvert_integer, deconvert_string};

use super::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Integer(isize),
    String(String),
    Boolean(bool),
    Variable(usize),
    UnaryOperator(String, Box<Node>),
    BinaryOperator(String, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Lambda(usize, Box<Node>),
}

impl Node {
    pub fn dump_tree(&self, indent: usize) {
        match self {
            Node::Integer(value) => println!("{:indent$}Integer({})", "", value, indent = indent),
            Node::String(value) => println!("{:indent$}String({})", "", value, indent = indent),
            Node::Boolean(value) => println!("{:indent$}Boolean({})", "", value, indent = indent),
            Node::Variable(value) => println!("{:indent$}Variable({})", "", value, indent = indent),
            Node::UnaryOperator(operator, operand) => {
                println!("{:indent$}UnaryOperator({})", "", operator, indent = indent);
                operand.dump_tree(indent + 2);
            }
            Node::BinaryOperator(operator, left, right) => {
                println!(
                    "{:indent$}BinaryOperator({})",
                    "",
                    operator,
                    indent = indent
                );
                left.dump_tree(indent + 2);
                right.dump_tree(indent + 2);
            }
            Node::If(condition, then_branch, else_branch) => {
                println!("{:indent$}If", "", indent = indent);
                condition.dump_tree(indent + 2);
                then_branch.dump_tree(indent + 2);
                else_branch.dump_tree(indent + 2);
            }
            Node::Lambda(arity, body) => {
                println!("{:indent$}Lambda({})", "", arity, indent = indent);
                body.dump_tree(indent + 2);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut tokens: Vec<Token> = vec![];
        fn traverse(node: &Node, tokens: &mut Vec<Token>) {
            match node {
                Node::Integer(value) => tokens.push(Token::Integer(*value as usize)),
                Node::String(value) => tokens.push(Token::String(value.clone())),
                Node::Boolean(value) => tokens.push(Token::Boolean(*value)),
                Node::Variable(value) => tokens.push(Token::Variable(*value)),
                Node::UnaryOperator(operator, operand) => {
                    tokens.push(Token::UnaryOperator(operator.clone()));
                    traverse(operand, tokens);
                }
                Node::BinaryOperator(operator, left, right) => {
                    tokens.push(Token::BinaryOperator(operator.clone()));
                    traverse(left, tokens);
                    traverse(right, tokens);
                }
                Node::If(condition, then_branch, else_branch) => {
                    tokens.push(Token::If);
                    traverse(condition, tokens);
                    traverse(then_branch, tokens);
                    traverse(else_branch, tokens);
                }
                Node::Lambda(arity, body) => {
                    tokens.push(Token::Lambda(*arity));
                    traverse(body, tokens);
                }
                _ => {}
            }
        }
        traverse(self, &mut tokens);
        tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
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
                Node::Integer(value as isize)
            }
            Token::String(ref value) => {
                self.position += 1;
                Node::String(value.clone())
            }
            Token::Boolean(value) => {
                self.position += 1;
                Node::Boolean(value)
            }
            Token::Variable(value) => {
                self.position += 1;
                Node::Variable(value)
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
        let arity = match self.tokens[self.position] {
            Token::Lambda(arity) => arity,
            _ => panic!("Expected lambda"),
        };
        self.position += 1;
        let body = Box::new(self.parse_node());
        Node::Lambda(arity, body)
    }
}

#[cfg(test)]
mod tests {
    use crate::icfp::transpiler::Transpiler;

    use super::*;

    #[test]
    fn test_parse_unary() {
        let tokens = vec![Token::UnaryOperator("-".to_string()), Token::Integer(3)];
        let mut parser = Parser::new(&tokens);
        let node = parser.parse_unary();
        assert_eq!(
            node,
            Node::UnaryOperator("-".to_string(), Box::new(Node::Integer(3)))
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
                    Box::new(Node::Integer(3))
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
                Box::new(Node::Integer(3)),
                Box::new(Node::Integer(4))
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
                Box::new(Node::Integer(3)),
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(4)),
                    Box::new(Node::Integer(5))
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
                    Box::new(Node::Integer(2)),
                    Box::new(Node::Integer(3))
                )),
                Box::new(Node::String("yes".to_string())),
                Box::new(Node::String("no".to_string()))
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
            Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(1)),
                    Box::new(Node::Integer(2))
                ))
            )
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
                    Box::new(Node::Lambda(
                        2,
                        Box::new(Node::Lambda(3, Box::new(Node::Variable(2))))
                    )),
                    Box::new(Node::BinaryOperator(
                        ".".to_string(),
                        Box::new(Node::String("Hello".to_string())),
                        Box::new(Node::String(" World!".to_string()))
                    ))
                )),
                Box::new(Node::Integer(42))
            )
        );
    }
}
