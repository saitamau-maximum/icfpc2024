use icfpc2024::icfp::{tokenizer::Tokenizer, util::deconvert_string};
use std::io::stdin;

fn main() {
    let text = {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer
    };
    println!("ENCRYPTED: S{}", deconvert_string(text.to_string()));
}
