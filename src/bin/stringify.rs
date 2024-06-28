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

    for token in result {
        println!("{:?}", token);
    }
}
