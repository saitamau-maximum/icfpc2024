use icfpc2024::icfp::evaluator::Evaluator;
use icfpc2024::icfp::parser::{Node, Parser};
use icfpc2024::icfp::tokenizer::Tokenizer;
use std::io::stdin;

fn main() {
    let text = {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer
    };
    let text = text.trim();
    let mut tokenizer = Tokenizer::new(text);
    let result = tokenizer.tokenize();
    let mut parser = Parser::new(&result);
    let result = parser.parse();
    let mut evaluator = Evaluator::new(result);
    let result = evaluator.evaluate();

    match result {
        Node::String(s) => println!("{}", s),
        Node::Integer(n) => println!("{}", n),
        _ => {
            println!("=== unexpected result ===");
            result.dump_tree(0);
            panic!("Unexpected result: {:?}", result)
        }
    }
}
