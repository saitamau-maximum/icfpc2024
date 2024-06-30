use std::collections::HashMap;

use super::parser::Node;
use super::util::{convert_integer, convert_string, deconvert_integer, deconvert_string};

pub struct Evaluator {
    node: Node,
}

impl Evaluator {
    pub fn new(node: Node) -> Evaluator {
        Evaluator { node }
    }

    pub fn evaluate(&self) -> Node {
        self.evaluate_node(&self.node)
    }

    fn evaluate_node(&self, node: &Node) -> Node {
        match node {
            Node::Integer(_) => node.clone(),
            Node::String(_) => node.clone(),
            Node::Boolean(_) => node.clone(),
            Node::Variable(_) => node.clone(),
            Node::Lambda(arity, body) => Node::Lambda(*arity, body.clone()),
            Node::UnaryOperator(operator, operand) => {
                self.evaluate_unary_operator(operator, *operand.clone())
            }
            Node::BinaryOperator(_, _, _) => self.evaluate_binary_operator(node.clone()),
            Node::If(condition, then_branch, else_branch) => {
                let condition = self.evaluate_node(condition);
                match condition {
                    Node::Boolean(true) => self.evaluate_node(then_branch),
                    Node::Boolean(false) => self.evaluate_node(else_branch),
                    _ => panic!("Unsupported condition: {:?}", condition),
                }
            }
            _ => panic!("Unsupported node: {:?}", node),
        }
    }

    fn evaluate_unary_operator(&self, operator: &str, operand: Node) -> Node {
        let operand = self.evaluate_node(&operand);
        match operator {
            "-" => match operand {
                Node::Integer(value) => Node::Integer(-value),
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            "!" => match operand {
                Node::Boolean(value) => Node::Boolean(!value),
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            // string to int
            "#" => match operand {
                Node::String(value) => {
                    let result = deconvert_string(value);
                    let result = convert_integer(result);
                    Node::Integer(result as isize)
                }
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            // int to string
            "$" => match operand {
                Node::Integer(value) => {
                    let result = deconvert_integer(value as usize);
                    let result = convert_string(result);
                    Node::String(result)
                }
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            _ => panic!("Unsupported unary operator: {}", operator),
        }
    }

    fn evaluate_binary_operator(&self, node: Node) -> Node {
        let (operator, left, right) = match node {
            Node::BinaryOperator(ref operator, ref left, ref right) => (operator, left, right),
            _ => panic!("Expected binary operator"),
        };
        let left = self.evaluate_node(left);
        let right = self.evaluate_node(right);
        match operator.as_str() {
            "+" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left + right),
                _ => node.clone(),
            },
            "-" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left - right),
                _ => node,
            },
            "*" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left * right),
                _ => node,
            },
            "/" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left / right),
                _ => node,
            },
            "%" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left % right),
                _ => node,
            },
            "<" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Boolean(left < right),
                _ => node,
            },
            ">" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Boolean(left > right),
                _ => node,
            },
            "=" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Boolean(left == right),
                (Node::String(left), Node::String(right)) => Node::Boolean(left == right),
                (Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left == right),
                _ => node,
            },
            "|" => match (left, right) {
                (Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left || right),
                _ => node,
            },
            "&" => match (left, right) {
                (Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left && right),
                _ => node,
            },
            "." => match (left, right) {
                (Node::String(left), Node::String(right)) => {
                    let result = left + &right;
                    Node::String(result)
                }
                _ => node,
            },
            "T" => match (left, right) {
                (Node::Integer(left), Node::String(right)) => {
                    let result = right.chars().take(left as usize).collect();
                    Node::String(result)
                }
                _ => node,
            },
            "D" => match (left, right) {
                (Node::Integer(left), Node::String(right)) => {
                    let result = right.chars().skip(left as usize).collect();
                    Node::String(result)
                }
                _ => node,
            },
            // apply term x to y, find variable using DFS, replace it with y and evaluate
            "$" => match (left, right) {
                (Node::Lambda(arity, body), arg) => {
                    let mut variables = HashMap::new();
                    variables.insert(arity, arg);
                    self.evaluate_node(&self.replace_variable(&body, &variables))
                }
                _ => node,
            },
            _ => panic!("Unsupported binary operator: {}", operator),
        }
    }

    fn replace_variable(&self, node: &Node, variables: &HashMap<usize, Node>) -> Node {
        match node {
            Node::Integer(_) => node.clone(),
            Node::String(_) => node.clone(),
            Node::Boolean(_) => node.clone(),
            Node::Variable(index) => match variables.get(index) {
                Some(value) => value.clone(),
                None => node.clone(),
            },
            Node::Lambda(arity, body) => {
                let mut new_variables = variables.clone();
                new_variables.insert(*arity, Node::Variable(*arity));
                let new_body = self.replace_variable(body, &new_variables);
                Node::Lambda(*arity, Box::new(new_body))
            }
            Node::UnaryOperator(operator, operand) => {
                let new_operand = self.replace_variable(operand, variables);
                Node::UnaryOperator(operator.clone(), Box::new(new_operand))
            }
            Node::BinaryOperator(operator, left, right) => {
                let new_left = self.replace_variable(left, variables);
                let new_right = self.replace_variable(right, variables);
                Node::BinaryOperator(operator.clone(), Box::new(new_left), Box::new(new_right))
            }
            Node::If(condition, then_branch, else_branch) => {
                let new_condition = self.replace_variable(condition, variables);
                let new_then_branch = self.replace_variable(then_branch, variables);
                let new_else_branch = self.replace_variable(else_branch, variables);
                Node::If(
                    Box::new(new_condition),
                    Box::new(new_then_branch),
                    Box::new(new_else_branch),
                )
            }
            _ => panic!("Unsupported node: {:?}", node),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::icfp::{parser::Parser, tokenizer::Tokenizer};

    use super::*;

    #[test]
    fn test_evaluate_unary_operator() {
        let evaluator = Evaluator::new(Node::Integer(42));
        assert_eq!(
            evaluator.evaluate_unary_operator("-", Node::Integer(42)),
            Node::Integer(-42)
        );
        assert_eq!(
            evaluator.evaluate_unary_operator("!", Node::Boolean(true)),
            Node::Boolean(false)
        );
        // assert_eq!(
        //     evaluator.evaluate_unary_operator("#", Node::String("4%34".to_string())),
        //     Node::Integer(15818151)
        // );
        assert_eq!(
            evaluator.evaluate_unary_operator("$", Node::Integer(15818151)),
            Node::String("test".to_string())
        );
    }

    #[test]
    fn test_evaluate_binary_operator() {
        let evaluator = Evaluator::new(Node::Integer(42));
        let cases = vec![
            ("+", Node::Integer(2), Node::Integer(3), Node::Integer(5)),
            ("-", Node::Integer(3), Node::Integer(2), Node::Integer(1)),
            ("*", Node::Integer(3), Node::Integer(2), Node::Integer(6)),
            ("/", Node::Integer(-7), Node::Integer(2), Node::Integer(-3)),
            ("%", Node::Integer(-7), Node::Integer(2), Node::Integer(-1)),
            (
                "<",
                Node::Integer(3),
                Node::Integer(2),
                Node::Boolean(false),
            ),
            (">", Node::Integer(3), Node::Integer(2), Node::Boolean(true)),
            (
                "=",
                Node::Integer(3),
                Node::Integer(2),
                Node::Boolean(false),
            ),
            (
                "|",
                Node::Boolean(true),
                Node::Boolean(false),
                Node::Boolean(true),
            ),
            (
                "&",
                Node::Boolean(true),
                Node::Boolean(false),
                Node::Boolean(false),
            ),
            (
                ".",
                Node::String("te".to_string()),
                Node::String("st".to_string()),
                Node::String("test".to_string()),
            ),
            (
                "T",
                Node::Integer(3),
                Node::String("test".to_string()),
                Node::String("tes".to_string()),
            ),
            (
                "D",
                Node::Integer(3),
                Node::String("test".to_string()),
                Node::String("t".to_string()),
            ),
        ];
        for (operator, left, right, expected) in cases {
            assert_eq!(
                evaluator.evaluate_binary_operator(Node::BinaryOperator(
                    operator.to_string(),
                    Box::new(left),
                    Box::new(right)
                )),
                expected
            );
        }
    }

    #[test]
    fn test_evaluate_lambda() {
        let evaluator = Evaluator::new(Node::Lambda(
            1,
            Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Integer(1)),
                Box::new(Node::Integer(2)),
            )),
        ));
        assert_eq!(
            evaluator.evaluate(),
            Node::Lambda(1, Box::new(Node::Integer(3)))
        );

        let evaluator = Evaluator::new(Node::Lambda(
            1,
            Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Integer(1)),
                Box::new(Node::Variable(2)),
            )),
        ));
        assert_eq!(
            evaluator.evaluate(),
            Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(1)),
                    Box::new(Node::Variable(2))
                ))
            )
        );

        let evaluator = Evaluator::new(Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(1)),
                    Box::new(Node::Variable(1)),
                )),
            )),
            Box::new(Node::Integer(42)),
        ));
        assert_eq!(evaluator.evaluate(), Node::Integer(43));

        let evaluator = Evaluator::new(Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::BinaryOperator(
                "$".to_string(),
                Box::new(Node::Lambda(
                    2,
                    Box::new(Node::Lambda(
                        1,
                        Box::new(Node::BinaryOperator(
                            "+".to_string(),
                            Box::new(Node::Variable(1)),
                            Box::new(Node::Variable(2)),
                        )),
                    )),
                )),
                Box::new(Node::Integer(42)),
            )),
            Box::new(Node::Integer(43)),
        ));
        assert_eq!(evaluator.evaluate(), Node::Integer(85));
    }

    #[test]
    fn test_evaluate() {
        let mut tokenizer = Tokenizer::new("B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK");
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(&tokens);
        let node = parser.parse();
        let evaluator = Evaluator::new(node);
        assert_eq!(
            evaluator.evaluate(),
            Node::String("Hello World!".to_string())
        );
    }
}
