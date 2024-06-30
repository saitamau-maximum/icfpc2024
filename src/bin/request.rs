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

    dotenv().ok();

    let env = Env {
        token: env::var("TOKEN").expect("TOKEN must be set"),
    };

    let client = reqwest::Client::new();

    let res = client
        .post(API_URL)
        .header("Authorization", format!("Bearer {}", env.token))
        .header("Content-Type", "text/plain")
        .body(text.to_string())
        .send()
        .await;

    match res {
        Ok(res) => {
            let body = res.text().await.unwrap();
            eprintln!("=== Response Start ===");
            eprintln!("{}", body);
            eprintln!("=== Response End ===");
            let tokens = Tokenizer::new(&body).tokenize();
            let node = Parser::new(&tokens).parse();
            let result = Evaluator::new(node).evaluate();
            eprintln!("=== Result Start ===");
            match result {
                Node::String(s) => println!("{}", s),
                _ => panic!("Unexpected result: {:?}", result),
            }
            eprintln!("=== Result End ===");
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
