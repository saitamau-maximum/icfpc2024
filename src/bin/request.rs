use dotenv::dotenv;
use icfpc2024::icfp::{
    evaluator::Evaluator,
    parser::{Node, Parser},
    tokenizer::Tokenizer,
    util::deconvert_string,
};
use reqwest;
use std::{env, io::stdin};

use serde::{Deserialize, Serialize};

const API_URL: &str = "https://boundvariable.space/communicate";
struct Env {
    token: String,
}

#[tokio::main]
async fn main() {
    let text = {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer
    };
    let text = text.trim();
    let mut tokenizer = Tokenizer::new(text);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(&tokens);
    let node = parser.parse();
    let evaluator = Evaluator::new(node);
    let result = evaluator.evaluate();
    eprintln!("=== Result Start ===");
    eprintln!("{:?}", result);
    eprintln!("=== Result End ===");

    let code = match result {
        Node::String(code) => code,
        _ => panic!("Result is not a string"),
    };

    let encoded = deconvert_string(code);
    let ans = "S".to_string() + &encoded;
    eprintln!("=== Ans Start ===");
    eprintln!("{}", ans);
    eprintln!("=== Ans End ===");

    dotenv().ok();

    let env = Env {
        token: env::var("TOKEN").expect("TOKEN must be set"),
    };

    let client = reqwest::Client::new();

    let res = client
        .post(API_URL)
        .header("Authorization", format!("Bearer {}", env.token))
        .header("Content-Type", "text/plain")
        .body(ans)
        .send()
        .await;

    match res {
        Ok(res) => {
            let body = res.text().await.unwrap();
            eprintln!("=== Response Start ===");
            eprintln!("{}", body);
            eprintln!("=== Response End ===");
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
