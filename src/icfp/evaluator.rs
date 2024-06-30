use std::collections::HashMap;

use num_bigint::BigInt;

use super::parser::Node;
use super::util::{
    convert_integer_to_bigint, convert_string, deconvert_integer_from_bigint, deconvert_string,
};

pub struct Evaluator {
    node: Node,
    cache: HashMap<Node, Node>,
    eval_count: usize,
}

impl Evaluator {
    pub fn new(node: Node) -> Evaluator {
        Evaluator {
            node,
            cache: HashMap::new(),
            eval_count: 0,
        }
    }

    pub fn evaluate(&mut self) -> Node {
        let result = self.evaluate_node(&self.node.clone());
        let result = match result {
            Node::String(_) => result,
            Node::Integer(_) => result,
            _ => self.evaluate_node(&result),
        };
        eprintln!("Evaluated {} nodes", self.eval_count);
        result
    }

    fn evaluate_node(&mut self, node: &Node) -> Node {
        if let Some(result) = self.cache.get(node) {
            return result.clone();
        }
        self.eval_count += 1;
        let result = match node {
            Node::Integer(_) => node.clone(),
            Node::String(_) => node.clone(),
            Node::Boolean(_) => node.clone(),
            Node::Variable(_) => node.clone(),
            Node::Lambda(arity, body) => Node::Lambda(arity.clone(), body.clone()),
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
        };
        // println!("======= evaluated node end: {} =======", result.name());
        self.cache.insert(node.clone(), result.clone());
        result
    }

    fn evaluate_unary_operator(&mut self, operator: &str, operand: Node) -> Node {
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
                    let result = convert_integer_to_bigint(result);
                    Node::Integer(result)
                }
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            // int to string
            "$" => match operand {
                Node::Integer(value) => {
                    let result = deconvert_integer_from_bigint(value);
                    let result = convert_string(result);
                    Node::String(result)
                }
                _ => panic!("Unsupported operand for unary operator: {:?}", operand),
            },
            _ => panic!("Unsupported unary operator: {}", operator),
        }
    }

    fn evaluate_binary_operator(&mut self, node: Node) -> Node {
        let (operator, left, right) = match node {
            Node::BinaryOperator(ref operator, ref left, ref right) => (operator, left, right),
            _ => panic!("Expected binary operator"),
        };
        // eprintln!("eval left before");
        // left.edump_tree(0);
        let left = self.evaluate_node(left);
        // eprintln!("eval left after");
        // left.edump_tree(0);
        // eprintln!("eval right before");
        // right.edump_tree(0);
        let right = self.evaluate_node(right);
        // eprintln!("eval right after");
        // right.edump_tree(0);
        // eprintln!("NODE_NAME: {}", node.name());
        match (operator.as_str(), left, right) {
            ("+", Node::Integer(left), Node::Integer(right)) => {
                let result = left + right;
                eprintln!("{}", result.to_string());
                Node::Integer(result)
            }
            ("-", Node::Integer(left), Node::Integer(right)) => Node::Integer(left - right),
            ("*", Node::Integer(left), Node::Integer(right)) => Node::Integer(left * right),
            ("/", Node::Integer(left), Node::Integer(right)) => {
                eprintln!("divided! / {}", right);
                let result = left / right;
                eprintln!("divided digit: {}", result.to_string().len());
                Node::Integer(result)
            }
            ("%", Node::Integer(left), Node::Integer(right)) => Node::Integer(left % right),
            ("<", Node::Integer(left), Node::Integer(right)) => Node::Boolean(left < right),
            (">", Node::Integer(left), Node::Integer(right)) => Node::Boolean(left > right),
            ("=", Node::Integer(left), Node::Integer(right)) => Node::Boolean(left == right),
            ("=", Node::String(left), Node::String(right)) => Node::Boolean(left == right),
            ("=", Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left == right),
            ("|", Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left || right),
            ("&", Node::Boolean(left), Node::Boolean(right)) => Node::Boolean(left && right),
            (".", Node::String(left), Node::String(right)) => {
                let result = left + &right;
                Node::String(result)
            }
            ("T", Node::Integer(left), Node::String(right)) => {
                let left_usize = left.to_string().parse::<usize>().unwrap();
                let result = right.chars().take(left_usize).collect();
                Node::String(result)
            }
            ("D", Node::Integer(left), Node::String(right)) => {
                let left_usize = left.to_string().parse::<usize>().unwrap();
                let result = right.chars().skip(left_usize).collect();
                Node::String(result)
            }
            ("$", Node::Lambda(_, _), Node::Lambda(_, _)) => {
                let mut node = self.apply_one_lambda(&node);
                // println!("START IMPOOOOOOOOOOOOOOOOOOOTANT");
                loop {
                    // node.dump_tree(0);
                    if !matches!(node, Node::BinaryOperator(_, _, _)) {
                        break;
                    }
                    if let Node::BinaryOperator(op, _, _) = node.clone() {
                        if op != "$" {
                            break;
                        }
                    }
                    node = self.apply_one_lambda(&node);
                }
                // println!("END IMPOOOOOOOOOOOOOOOOOOOTANT");
                // node.dump_tree(0);
                node
            }
            ("$", Node::Lambda(arity, body), arg) => {
                let mut variables = HashMap::new();
                variables.insert(arity, arg);
                let new_body = self.replace_variable(&body, &variables);
                self.evaluate_node(&new_body)
            }
            _ => node,
        }
    }

    // 1段階だけ適用する、再帰的には適用するとStack Overflowになる
    fn apply_one_lambda(&mut self, node: &Node) -> Node {
        // println!("======= applying one lambda =======");
        assert!(matches!(node, Node::BinaryOperator(_, _, _)));
        let (operator, left, right) = match node {
            Node::BinaryOperator(operator, left, right) => (operator, left, right),
            _ => panic!("Expected binary operator"),
        };
        assert_eq!(operator, "$");
        let (lambda, body, arg) = match (left.as_ref(), right.as_ref()) {
            (Node::Lambda(arity, body), arg) => (arity, body, arg),
            _ => panic!("The left side of $ operator must be a lambda"),
        };
        let mut variables = HashMap::new();
        variables.insert(lambda.clone(), arg.clone());
        self.replace_variable(&body, &variables)
    }

    fn replace_variable(&mut self, node: &Node, variables: &HashMap<usize, Node>) -> Node {
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
                new_variables.insert(arity.clone(), Node::Variable(arity.clone()));
                let new_body = self.replace_variable(body, &new_variables);
                Node::Lambda(arity.clone(), Box::new(new_body))
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
    use crate::{
        icfp::{parser::Parser, tokenizer::Tokenizer},
        node,
    };

    use super::*;

    #[test]
    fn test_evaluate_unary_operator() {
        let mut evaluator = Evaluator::new(Node::Integer(BigInt::from(42)));
        assert_eq!(
            evaluator.evaluate_unary_operator("-", Node::Integer(BigInt::from(42))),
            Node::Integer(BigInt::from(-42))
        );
        assert_eq!(
            evaluator.evaluate_unary_operator("!", Node::Boolean(true)),
            Node::Boolean(false)
        );
        // assert_eq!(
        //     evaluator.evaluate_unary_operator("#", Node::String("4%34".to_string())),
        //     Node::Integer(BigInt::from(15818151))
        // );
        assert_eq!(
            evaluator.evaluate_unary_operator("$", Node::Integer(BigInt::from(15818151))),
            Node::String("test".to_string())
        );
    }

    #[test]
    fn test_evaluate_binary_operator() {
        let mut evaluator = Evaluator::new(Node::Integer(BigInt::from(42)));
        let cases = vec![
            (
                "+",
                Node::Integer(BigInt::from(2)),
                Node::Integer(BigInt::from(3)),
                Node::Integer(BigInt::from(5)),
            ),
            (
                "-",
                Node::Integer(BigInt::from(3)),
                Node::Integer(BigInt::from(2)),
                Node::Integer(BigInt::from(1)),
            ),
            (
                "*",
                Node::Integer(BigInt::from(3)),
                Node::Integer(BigInt::from(2)),
                Node::Integer(BigInt::from(6)),
            ),
            (
                "/",
                Node::Integer(BigInt::from(-7)),
                Node::Integer(BigInt::from(2)),
                Node::Integer(BigInt::from(-3)),
            ),
            (
                "%",
                Node::Integer(BigInt::from(-7)),
                Node::Integer(BigInt::from(2)),
                Node::Integer(BigInt::from(-1)),
            ),
            (
                "<",
                Node::Integer(BigInt::from(3)),
                Node::Integer(BigInt::from(2)),
                Node::Boolean(false),
            ),
            (
                ">",
                Node::Integer(BigInt::from(3)),
                Node::Integer(BigInt::from(2)),
                Node::Boolean(true),
            ),
            (
                "=",
                Node::Integer(BigInt::from(3)),
                Node::Integer(BigInt::from(2)),
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
                Node::Integer(BigInt::from(3)),
                Node::String("test".to_string()),
                Node::String("tes".to_string()),
            ),
            (
                "D",
                Node::Integer(BigInt::from(3)),
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
        let mut evaluator = Evaluator::new(Node::Lambda(
            1,
            Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Integer(BigInt::from(1))),
                Box::new(Node::Integer(BigInt::from(2))),
            )),
        ));
        assert_eq!(
            evaluator.evaluate(),
            Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(BigInt::from(1))),
                    Box::new(Node::Integer(BigInt::from(2)))
                ))
            )
        );

        let mut evaluator = Evaluator::new(Node::Lambda(
            1,
            Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Integer(BigInt::from(1))),
                Box::new(Node::Variable(2)),
            )),
        ));
        assert_eq!(
            evaluator.evaluate(),
            Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(BigInt::from(1))),
                    Box::new(Node::Variable(2))
                ))
            )
        );

        let mut evaluator = Evaluator::new(Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(BigInt::from(1))),
                    Box::new(Node::Variable(1)),
                )),
            )),
            Box::new(Node::Integer(BigInt::from(42))),
        ));
        assert_eq!(evaluator.evaluate(), Node::Integer(BigInt::from(43)));

        let mut evaluator = Evaluator::new(Node::BinaryOperator(
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
                Box::new(Node::Integer(BigInt::from(42))),
            )),
            Box::new(Node::Integer(BigInt::from(43))),
        ));
        assert_eq!(evaluator.evaluate(), Node::Integer(BigInt::from(85)));
    }

    #[test]
    fn test_evaluate() {
        let mut tokenizer = Tokenizer::new("B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK");
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(&tokens);
        let node = parser.parse();
        let mut evaluator = Evaluator::new(node);
        assert_eq!(
            evaluator.evaluate(),
            Node::String("Hello World!".to_string())
        );
    }

    #[test]
    fn test_apply_one_lambda() {
        let mut evaluator = Evaluator::new(Node::String("test".to_string()));
        let result = evaluator.apply_one_lambda(&Node::BinaryOperator(
            "$".to_string(),
            node!(Node::Lambda(
                1,
                node!(Node::BinaryOperator(
                    "$".to_string(),
                    node!(Node::Variable(1)),
                    node!(Node::Integer(BigInt::from(1))),
                )),
            )),
            node!(Node::Lambda(
                1,
                node!(Node::BinaryOperator(
                    "$".to_string(),
                    node!(Node::Variable(1)),
                    node!(Node::Integer(BigInt::from(2))),
                )),
            )),
        ));

        assert_eq!(
            result,
            Node::BinaryOperator(
                "$".to_string(),
                node!(Node::Lambda(
                    1,
                    node!(Node::BinaryOperator(
                        "$".to_string(),
                        node!(Node::Variable(1)),
                        node!(Node::Integer(BigInt::from(2))),
                    )),
                )),
                node!(Node::Integer(BigInt::from(1))),
            )
        );
    }

    #[test]
    fn test_prod() {
        let node = Node::BinaryOperator(
            "$".to_string(),
            node!(Node::Lambda(
                1,
                node!(Node::BinaryOperator(
                    "$".to_string(),
                    node!(Node::Lambda(
                        2,
                        node!(Node::BinaryOperator(
                            "$".to_string(),
                            node!(Node::Variable(1)),
                            node!(Node::BinaryOperator(
                                "$".to_string(),
                                node!(Node::Variable(2)),
                                node!(Node::Variable(2)),
                            )),
                        )),
                    )),
                    node!(Node::Lambda(
                        2,
                        node!(Node::BinaryOperator(
                            "$".to_string(),
                            node!(Node::Variable(1)),
                            node!(Node::BinaryOperator(
                                "$".to_string(),
                                node!(Node::Variable(2)),
                                node!(Node::Variable(2)),
                            )),
                        )),
                    )),
                )),
            )),
            node!(Node::Lambda(
                3,
                node!(Node::Lambda(
                    2,
                    node!(Node::If(
                        node!(Node::BinaryOperator(
                            "=".to_string(),
                            node!(Node::Variable(2)),
                            node!(Node::Integer(BigInt::from(1))),
                        )),
                        node!(Node::Variable(1)),
                        node!(Node::BinaryOperator(
                            ".".to_string(),
                            node!(Node::Variable(1)),
                            node!(Node::BinaryOperator(
                                "$".to_string(),
                                node!(Node::Variable(3)),
                                node!(Node::BinaryOperator(
                                    "-".to_string(),
                                    node!(Node::Variable(2)),
                                    node!(Node::Integer(BigInt::from(1))),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        );
        let mut evaluator = Evaluator::new(Node::String("test".to_string()));
        let result = evaluator.apply_one_lambda(&node);
        result.dump_tree(0);
    }
}
