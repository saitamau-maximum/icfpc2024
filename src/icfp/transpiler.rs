use super::parser::Node;

// convert the AST to a common lisp program
pub struct Transpiler {
    node: Node,
}

impl Transpiler {
    pub fn new(node: Node) -> Self {
        Self { node }
    }

    pub fn transpile(&self) -> String {
        self.transpile_node(self.node.clone())
    }

    fn transpile_node(&self, node: Node) -> String {
        match node {
            Node::Integer(value) => value.to_string(),
            Node::String(value) => format!("\"{}\"", value),
            Node::Boolean(value) => format!("{}", if value { "1" } else { "nil" }),
            Node::Variable(value) => format!("v{}", value),
            Node::UnaryOperator(operator, operand) => match operator.as_str() {
                "-" => format!("-{}", self.transpile_node(*operand)),
                "!" => format!(
                    "{}",
                    if operand.as_ref() == &Node::Boolean(true) {
                        "nil"
                    } else {
                        "1"
                    }
                ),
                _ => format!("{} {}", operator, self.transpile_node(*operand)),
            },
            Node::BinaryOperator(operator, left, right) => match operator.as_str() {
                "." => format!(
                    "(concatenate 'string {} {})",
                    self.transpile_node(*left),
                    self.transpile_node(*right)
                ),
                "$" => format!(
                    // "({} {})",
                    "(funcall {} {})",
                    self.transpile_node(*left),
                    self.transpile_node(*right)
                ),
                "=" => format!(
                    "(== {} {})",
                    self.transpile_node(*left),
                    self.transpile_node(*right)
                ),
                _ => format!(
                    "({} {} {})",
                    operator,
                    self.transpile_node(*left),
                    self.transpile_node(*right)
                ),
            },
            Node::If(condition, then_branch, else_branch) => format!(
                "(if {} {} {})",
                self.transpile_node(*condition),
                self.transpile_node(*then_branch),
                self.transpile_node(*else_branch)
            ),
            Node::Lambda(arity, body) => format!(
                "(lambda (v{}) {})",
                arity,
                self.transpile_node(body.as_ref().clone()),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
    use rust_lisp::default_env;
    use rust_lisp::interpreter::eval;
    use rust_lisp::model::Value;
    use rust_lisp::parser::parse;

    #[test]
    fn test_transpile() {
        let node = Node::BinaryOperator(
            "+".to_string(),
            Box::new(Node::Integer(1)),
            Box::new(Node::Integer(2)),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(+ 1 2)");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert_eq!(result, Value::Int(3));
    }

    #[test]
    fn test_transpile_nested() {
        let node = Node::BinaryOperator(
            "+".to_string(),
            Box::new(Node::Integer(1)),
            Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Integer(2)),
                Box::new(Node::Integer(3)),
            )),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(+ 1 (+ 2 3))");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert_eq!(result, Value::Int(6));
    }

    #[test]
    fn test_transpile_if() {
        let node = Node::If(
            Box::new(Node::Boolean(true)),
            Box::new(Node::Integer(1)),
            Box::new(Node::Integer(2)),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(if 1 1 2)");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert_eq!(result, Value::Int(1));

        let node = Node::If(
            Box::new(Node::Boolean(false)),
            Box::new(Node::Integer(1)),
            Box::new(Node::Integer(2)),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(if nil 1 2)");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert_eq!(result, Value::Int(2));
    }

    #[test]
    fn test_transpile_lambda() {
        let node = Node::Lambda(
            2,
            Box::new(Node::BinaryOperator(
                "+".to_string(),
                Box::new(Node::Integer(1)),
                Box::new(Node::Variable(2)),
            )),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(lambda (v2) (+ 1 v2))");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert!(matches!(result, Value::Lambda(_)));
    }

    #[test]
    fn test_transpile_nested_lambda() {
        let node = Node::Lambda(
            2,
            Box::new(Node::Lambda(
                3,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Variable(2)),
                    Box::new(Node::Variable(3)),
                )),
            )),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(lambda (v2) (lambda (v3) (+ v2 v3)))");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert!(matches!(result, Value::Lambda(_)));
    }

    #[test]
    fn test_transpile_unary_operator() {
        let node = Node::UnaryOperator("-".to_string(), Box::new(Node::Integer(1)));
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "-1");
        let env = Rc::new(RefCell::new(default_env()));
        let mut ast_iter = parse(&result);
        let ast = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &ast).unwrap();
        assert_eq!(result, Value::Int(-1));

        let node = Node::UnaryOperator("!".to_string(), Box::new(Node::Boolean(true)));
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "nil");
    }

    #[test]
    fn test_integration() {
        let node = Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::Lambda(
                3,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Integer(4)),
                    Box::new(Node::Variable(3)),
                )),
            )),
            Box::new(Node::Integer(1)),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(result, "(funcall (lambda (v3) (+ 4 v3)) 1)");

        let node = Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::Lambda(
                2,
                Box::new(Node::BinaryOperator(
                    "$".to_string(),
                    Box::new(Node::Lambda(
                        3,
                        Box::new(Node::BinaryOperator(
                            "+".to_string(),
                            Box::new(Node::Variable(2)),
                            Box::new(Node::Variable(3)),
                        )),
                    )),
                    Box::new(Node::Integer(1)),
                )),
            )),
            Box::new(Node::Integer(2)),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(
            result,
            "(funcall (lambda (v2) (funcall (lambda (v3) (+ v2 v3)) 1)) 2)"
        );
    }

    #[test]
    fn test_more_complex() {
        let node = Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::Lambda(
                2,
                Box::new(Node::BinaryOperator(
                    "$".to_string(),
                    Box::new(Node::Lambda(
                        3,
                        Box::new(Node::BinaryOperator(
                            "+".to_string(),
                            Box::new(Node::Variable(2)),
                            Box::new(Node::Variable(3)),
                        )),
                    )),
                    Box::new(Node::Integer(1)),
                )),
            )),
            Box::new(Node::BinaryOperator(
                "$".to_string(),
                Box::new(Node::Lambda(
                    3,
                    Box::new(Node::BinaryOperator(
                        "+".to_string(),
                        Box::new(Node::Integer(4)),
                        Box::new(Node::Variable(3)),
                    )),
                )),
                Box::new(Node::Integer(2)),
            )),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(
            result,
            "(funcall (lambda (v2) (funcall (lambda (v3) (+ v2 v3)) 1)) (funcall (lambda (v3) (+ 4 v3)) 2))"
        );
    }

    #[test]
    fn test_lambda_variable() {
        let node = Node::BinaryOperator(
            "$".to_string(),
            Box::new(Node::Lambda(
                2,
                Box::new(Node::BinaryOperator(
                    "$".to_string(),
                    Box::new(Node::Variable(2)),
                    Box::new(Node::Integer(2)),
                )),
            )),
            Box::new(Node::Lambda(
                1,
                Box::new(Node::BinaryOperator(
                    "+".to_string(),
                    Box::new(Node::Variable(1)),
                    Box::new(Node::Integer(3)),
                )),
            )),
        );
        let transpiler = Transpiler::new(node);
        let result = transpiler.transpile();
        assert_eq!(
            result,
            "(funcall (lambda (v2) (funcall v2 2)) (lambda (v1) (+ v1 3)))"
        );
    }
}
