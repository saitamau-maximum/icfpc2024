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
            Node::UnaryOperator((operator, operand)) => {
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
    fn test_evaluate() {
        let evaluator = Evaluator::new(Node::UnaryOperator((
            "-".to_string(),
            Box::new(Node::Primitive(Primitive::Integer(42))),
        )));
        assert_eq!(evaluator.evaluate(), Primitive::Integer(-42));
    }
}
