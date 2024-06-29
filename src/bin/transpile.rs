use icfpc2024::icfp::{parser::Parser, tokenizer::Tokenizer, transpiler::Transpiler};
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
    let transpiler = Transpiler::new(result);
    let result = transpiler.transpile();

    println!("{}", result);
}
