use icfpc2024::{
    icfp::{
        builtin::{repeat_operator, repeat_string, y_combinator},
        parser::Node,
    },
    node,
};
use std::{collections::VecDeque, io::stdin};

fn get_compress_collection(text: &str) -> VecDeque<(char, usize)> {
    let mut compress_collection = VecDeque::new();
    let mut last_char = None;
    let mut count = 0;
    for c in text.chars() {
        if let Some(last) = last_char {
            if last == c {
                count += 1;
            } else {
                compress_collection.push_back((last, count));
                count = 1;
            }
        } else {
            count = 1;
        }
        last_char = Some(c);
    }
    if let Some(last) = last_char {
        compress_collection.push_back((last, count));
    }
    compress_collection
}

fn main() {
    let text = {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer
    };
    let text = text.trim();
    let mut compress_collection = get_compress_collection(text);

    fn generate_compress_operation_node(
        compress_collection: &mut VecDeque<(char, usize)>,
    ) -> Option<Node> {
        if let Some((c, count)) = compress_collection.pop_front() {
            let operator = repeat_operator(c, count);
            let compressed_string_node =
                Node::BinaryOperator("$".to_string(), node!(Node::Variable(0)), node!(operator));
            let full_string_node = Node::String(c.to_string().repeat(count));
            let string_node = if full_string_node.to_string().len()
                <= compressed_string_node.to_string().len() + 4
            {
                full_string_node
            } else {
                compressed_string_node
            };
            if let Some(child_compress_operation_node) =
                generate_compress_operation_node(compress_collection)
            {
                Some(Node::BinaryOperator(
                    ".".to_string(),
                    node!(string_node),
                    node!(child_compress_operation_node),
                ))
            } else {
                Some(string_node)
            }
        } else {
            None
        }
    }

    let compression_operation_node = generate_compress_operation_node(&mut compress_collection)
        .expect("compress_collection is empty");

    let node = node!(Node::BinaryOperator(
        "$".to_string(),
        node!(Node::Lambda(0, node!(compression_operation_node))),
        node!(Node::BinaryOperator(
            "$".to_string(),
            node!(y_combinator()),
            node!(repeat_string())
        )),
    ));

    let full_operation = node!(Node::String(text.to_string()));

    if full_operation.to_string().len() <= node.to_string().len() {
        println!("{}", full_operation.to_string());
    } else {
        println!("{}", node.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_compress_collection() {
        let text = "aaabbbccc";
        let compress_collection = get_compress_collection(text);
        assert_eq!(compress_collection, vec![('a', 3), ('b', 3), ('c', 3)]);
    }
}
