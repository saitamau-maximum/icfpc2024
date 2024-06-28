use crate::parser::Node;
use crate::util::{convert_integer, convert_string, deconvert_integer};

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
            Node::UnaryOperator(operator, operand) => {
                let result = self.evaluate_node(operand);
                self.evaluate_unary_operator(operator, result)
            }
            _ => panic!("Unsupported node: {:?}", node),
        }
    }

    fn evaluate_unary_operator(&self, operator: &str, operand: Node) -> Node {
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
                    let result = convert_integer(value);
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

    fn evaluate_binary_operator(&self, operator: &str, left: Node, right: Node) -> Node {
        match operator {
            "+" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left + right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "-" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left - right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "*" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left * right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "/" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left / right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "%" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Integer(left % right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "<" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Boolean(left < right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            ">" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Boolean(left > right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "=" => match (left, right) {
                (Node::Integer(left), Node::Integer(right)) => Node::Boolean(left == right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "|" => match (left, right) {
                (Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left || right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "&" => match (left, right) {
                (Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left && right),
                _ => panic!("Unsupported operands for binary operator"),
            },
            "." => match (left, right) {
                (Node::String(left), Node::String(right)) => {
                    let result = left + &right;
                    Node::String(result)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "T" => match (left, right) {
                (Node::String(left), Node::Integer(right)) => {
                    let result = left.chars().take(right as usize).collect();
                    Node::String(result)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "D" => match (left, right) {
                (Node::String(left), Node::Integer(right)) => {
                    let result = left.chars().skip(right as usize).collect();
                    Node::String(result)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            // apply term x to y
            "$" => match (left, right) {
                _ => panic!("Unsupported operands for binary operator"),
            },
            _ => panic!("Unsupported binary operator: {}", operator),
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(
            evaluator.evaluate_unary_operator("#", Node::String("4%34".to_string())),
            Node::Integer(15818151)
        );
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
                Node::String("test".to_string()),
                Node::Integer(3),
                Node::String("tes".to_string()),
            ),
            (
                "D",
                Node::String("test".to_string()),
                Node::Integer(3),
                Node::String("t".to_string()),
            ),
        ];
        for (operator, left, right, expected) in cases {
            assert_eq!(
                evaluator.evaluate_binary_operator(operator, left, right),
                expected
            );
        }
    }

    #[test]
    fn test_evaluate() {
        let evaluator = Evaluator::new(Node::UnaryOperator(
            "-".to_string(),
            Box::new(Node::Integer(42)),
        ));
        assert_eq!(evaluator.evaluate(), Node::Integer(-42));
    }
}
