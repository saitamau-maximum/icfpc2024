use crate::parser::{Node, Primitive};
use crate::util::{convert_integer, convert_string, deconvert_integer};

pub struct Evaluator {
    node: Node,
}

impl Evaluator {
    pub fn new(node: Node) -> Evaluator {
        Evaluator { node }
    }

    pub fn evaluate(&self) -> Primitive {
        self.evaluate_node(&self.node)
    }

    fn evaluate_node(&self, node: &Node) -> Primitive {
        match node {
            Node::Primitive(primitive) => primitive.clone(),
            Node::UnaryOperator(operator, operand) => {
                let result = self.evaluate_node(operand);
                self.evaluate_unary_operator(operator, result)
            }
            _ => panic!("Unsupported node: {:?}", node),
        }
    }

    fn evaluate_unary_operator(&self, operator: &str, operand: Primitive) -> Primitive {
        match operator {
            "-" => match operand {
                Primitive::Integer(value) => Primitive::Integer(-value),
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            "!" => match operand {
                Primitive::Boolean(value) => Primitive::Boolean(!value),
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            // string to int
            "#" => match operand {
                Primitive::String(value) => {
                    let result = convert_integer(value);
                    Primitive::Integer(result as isize)
                }
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            // int to string
            "$" => match operand {
                Primitive::Integer(value) => {
                    let result = deconvert_integer(value as usize);
                    let result = convert_string(result);
                    Primitive::String(result)
                }
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            _ => panic!("Unsupported unary operator: {}", operator),
        }
    }

    fn evaluate_binary_operator(
        &self,
        operator: &str,
        left: Primitive,
        right: Primitive,
    ) -> Primitive {
        match operator {
            "+" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Integer(left + right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "-" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Integer(left - right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "*" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Integer(left * right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "/" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Integer(left / right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "%" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Integer(left % right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "<" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Boolean(left < right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            ">" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Boolean(left > right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "=" => match (left, right) {
                (Primitive::Integer(left), Primitive::Integer(right)) => {
                    Primitive::Boolean(left == right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "|" => match (left, right) {
                (Primitive::Boolean(left), Primitive::Boolean(right)) => {
                    Primitive::Boolean(left || right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "&" => match (left, right) {
                (Primitive::Boolean(left), Primitive::Boolean(right)) => {
                    Primitive::Boolean(left && right)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "." => match (left, right) {
                (Primitive::String(left), Primitive::String(right)) => {
                    let result = left + &right;
                    Primitive::String(result)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "T" => match (left, right) {
                (Primitive::String(left), Primitive::Integer(right)) => {
                    let result = left.chars().take(right as usize).collect();
                    Primitive::String(result)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            "D" => match (left, right) {
                (Primitive::String(left), Primitive::Integer(right)) => {
                    let result = left.chars().skip(right as usize).collect();
                    Primitive::String(result)
                }
                _ => panic!("Unsupported operands for binary operator"),
            },
            // apply term x to y
            "$" => match (left, right) {
                // (Primitive::Lambda(left), right) => {
                //     let result = left.apply(right);
                //     result
                // }
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
        let evaluator = Evaluator::new(Node::Primitive(Primitive::Integer(42)));
        assert_eq!(
            evaluator.evaluate_unary_operator("-", Primitive::Integer(42)),
            Primitive::Integer(-42)
        );
        assert_eq!(
            evaluator.evaluate_unary_operator("!", Primitive::Boolean(true)),
            Primitive::Boolean(false)
        );
        assert_eq!(
            evaluator.evaluate_unary_operator("#", Primitive::String("4%34".to_string())),
            Primitive::Integer(15818151)
        );
        assert_eq!(
            evaluator.evaluate_unary_operator("$", Primitive::Integer(15818151)),
            Primitive::String("test".to_string())
        );
    }

    #[test]
    fn test_evaluate_binary_operator() {
        let evaluator = Evaluator::new(Node::Primitive(Primitive::Integer(42)));
        let cases = vec![
            (
                "+",
                Primitive::Integer(2),
                Primitive::Integer(3),
                Primitive::Integer(5),
            ),
            (
                "-",
                Primitive::Integer(3),
                Primitive::Integer(2),
                Primitive::Integer(1),
            ),
            (
                "*",
                Primitive::Integer(3),
                Primitive::Integer(2),
                Primitive::Integer(6),
            ),
            (
                "/",
                Primitive::Integer(-7),
                Primitive::Integer(2),
                Primitive::Integer(-3),
            ),
            (
                "%",
                Primitive::Integer(-7),
                Primitive::Integer(2),
                Primitive::Integer(-1),
            ),
            (
                "<",
                Primitive::Integer(3),
                Primitive::Integer(2),
                Primitive::Boolean(false),
            ),
            (
                ">",
                Primitive::Integer(3),
                Primitive::Integer(2),
                Primitive::Boolean(true),
            ),
            (
                "=",
                Primitive::Integer(3),
                Primitive::Integer(2),
                Primitive::Boolean(false),
            ),
            (
                "|",
                Primitive::Boolean(true),
                Primitive::Boolean(false),
                Primitive::Boolean(true),
            ),
            (
                "&",
                Primitive::Boolean(true),
                Primitive::Boolean(false),
                Primitive::Boolean(false),
            ),
            (
                ".",
                Primitive::String("te".to_string()),
                Primitive::String("st".to_string()),
                Primitive::String("test".to_string()),
            ),
            (
                "T",
                Primitive::String("test".to_string()),
                Primitive::Integer(3),
                Primitive::String("tes".to_string()),
            ),
            (
                "D",
                Primitive::String("test".to_string()),
                Primitive::Integer(3),
                Primitive::String("t".to_string()),
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
            Box::new(Node::Primitive(Primitive::Integer(42))),
        ));
        assert_eq!(evaluator.evaluate(), Primitive::Integer(-42));
    }
}
