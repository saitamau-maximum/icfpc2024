use icfpc2024::tokenizer::Tokenizer;
use proconio::input;

fn main() {
    input! {
        text: String,
    }
    let mut tokenizer = Tokenizer::new(&text);
    let result = tokenizer.tokenize();

    for token in result {
        println!("{:?}", token);
    }
}
