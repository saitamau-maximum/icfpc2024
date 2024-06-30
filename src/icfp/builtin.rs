use crate::icfp::util::STRING_ASCII;

use super::{
    parser::Node,
    util::{convert_string, INTEGER_ASCII},
};

#[macro_export]
macro_rules! node {
    ($node:expr) => {
        Box::new($node)
    };
}

pub fn y_combinator() -> Node {
    Node::Lambda(
        0,
        node!(Node::BinaryOperator(
            "$".to_string(),
            node!(Node::Lambda(
                1,
                node!(Node::BinaryOperator(
                    "$".to_string(),
                    node!(Node::Variable(1)),
                    node!(Node::Variable(1))
                ))
            )),
            node!(Node::Lambda(
                1,
                node!(Node::BinaryOperator(
                    "$".to_string(),
                    node!(Node::Variable(0)),
                    node!(Node::Lambda(
                        2,
                        node!(Node::BinaryOperator(
                            "$".to_string(),
                            node!(Node::BinaryOperator(
                                "$".to_string(),
                                node!(Node::Variable(1)),
                                node!(Node::Variable(1))
                            )),
                            node!(Node::Variable(2))
                        ))
                    ))
                ))
            ))
        )),
    )
}

const BASE94: isize = 94;

pub fn repeat_char() -> Node {
    Node::Lambda(
        0,
        node!(Node::Lambda(
            1,
            node!(Node::If(
                node!(Node::BinaryOperator(
                    ">".to_string(),
                    node!(Node::Variable(1)),
                    node!(Node::Integer(BASE94)) // 1文字以上からじゃないと出力できないようにする（ICFPに空文字列が存在しないため）
                )),
                node!(Node::BinaryOperator(
                    ".".to_string(),
                    node!(Node::UnaryOperator(
                        "$".to_string(),
                        node!(Node::BinaryOperator(
                            "%".to_string(),
                            node!(Node::Variable(1)),
                            node!(Node::Integer(BASE94))
                        ))
                    )),
                    node!(Node::BinaryOperator(
                        "$".to_string(),
                        node!(Node::Variable(0)),
                        node!(Node::BinaryOperator(
                            "-".to_string(),
                            node!(Node::Variable(1)),
                            node!(Node::Integer(BASE94))
                        ))
                    ))
                )),
                node!(Node::String("".to_string()))
            ))
        )),
    )
}

pub fn repeat_char_operator(value: char, times: usize) -> Node {
    assert!(times >= 1);
    let value_id = STRING_ASCII.find(value).unwrap();
    Node::Integer(BASE94 * times as isize + value_id as isize)
}

#[cfg(test)]
mod tests {
    use crate::icfp::transpiler::Transpiler;

    use super::*;

    #[test]
    fn test_builtin() {
        let node = node!(Node::BinaryOperator(
            "$".to_string(),
            node!(Node::BinaryOperator(
                "$".to_string(),
                node!(y_combinator()),
                node!(repeat_char())
            )),
            node!(Node::Integer(BASE94 * 1 + 5))
        ));
        node.dump_tree(0);
        eprintln!("{}", node.to_string());
        let transpiler = Transpiler::new(node.as_ref().clone());
        let result = transpiler.transpile();
        eprintln!("{}", result);
    }

    #[test]
    fn test_repeat_operator() {
        let node = repeat_char_operator('a', 3);
        assert_eq!(node, Node::Integer(282));
    }
}
