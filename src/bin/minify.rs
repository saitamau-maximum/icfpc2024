use icfpc2024::{
    icfp::{
        builtin::{repeat_char, repeat_char_operator, y_combinator},
        parser::Node,
    },
    node,
};
use std::{collections::VecDeque, io::stdin, iter::repeat};

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
            let mut compressing_str = String::new();
            let mut final_c = c;
            let mut final_count = count;
            while final_count <= 12 {
                compressing_str.push_str(&repeat(final_c).take(final_count).collect::<String>());
                if let Some((next_c, next_count)) = compress_collection.pop_front() {
                    final_c = next_c;
                    final_count = next_count;
                } else {
                    break;
                }
            }
            let operator = repeat_char_operator(final_c, final_count);
            let compressed_string_node =
                Node::BinaryOperator("$".to_string(), node!(Node::Variable(0)), node!(operator));
            let next_node = if let Some(child_compress_operation_node) =
                generate_compress_operation_node(compress_collection)
            {
                Node::BinaryOperator(
                    ".".to_string(),
                    node!(compressed_string_node),
                    node!(child_compress_operation_node),
                )
            } else {
                compressed_string_node
            };
            if compressing_str.is_empty() {
                Some(next_node)
            } else {
                let concatenated_node = Node::BinaryOperator(
                    ".".to_string(),
                    node!(Node::String(compressing_str)),
                    node!(next_node),
                );
                Some(concatenated_node)
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
            node!(repeat_char())
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
