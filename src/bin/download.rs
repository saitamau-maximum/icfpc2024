use dotenv::dotenv;
use icfpc2024::icfp::{
    evaluator::Evaluator,
    parser::{Node, Parser},
    tokenizer::Tokenizer,
    util::{convert_string, deconvert_string},
};
use reqwest;
use std::time;
use std::{env, io::stdin};

use serde::{Deserialize, Serialize};

// =========== CONFIG ===========
const API_URL: &str = "https://boundvariable.space/communicate";
const COMMAND: &str = "get lambdaman";
const SAVE_DIR: &str = "downloads";
const START_ID: usize = 9;
const END_ID: usize = 9;
// ==============================

struct Env {
    token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let env = Env {
        token: env::var("TOKEN").expect("TOKEN must be set"),
    };

    let client = reqwest::Client::new();

    std::fs::create_dir_all(SAVE_DIR).unwrap();
    for i in START_ID..=END_ID {
        eprintln!("downloading id: {}", i);
        let text = deconvert_string(format!("{}{}", COMMAND, i));
        let command = format!("S{}", text);

        let res = client
            .post(API_URL)
            .header("Authorization", format!("Bearer {}", env.token))
            .header("Content-Type", "text/plain")
            .body(command)
            .send()
            .await;

        match res {
            Ok(res) => {
                let body = res.text().await.unwrap();
                eprintln!("body: {}", body);
                let tokens = Tokenizer::new(&body).tokenize();
                let node = Parser::new(&tokens).parse();
                let result = Evaluator::new(node).evaluate();
                let text = match result {
                    Node::String(text) => text,
                    _ => panic!("Result is not a string"),
                };
                std::fs::write(format!("{}/{}.txt", SAVE_DIR, i), text).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }

        // 3秒待つ
        tokio::time::sleep(time::Duration::from_secs(3)).await;
    }
}
