use dotenv::dotenv;
use icfpc2024::icfp::{
    evaluator::Evaluator,
    parser::{Node, Parser},
    tokenizer::Tokenizer,
    util::deconvert_string,
};
use reqwest;
use std::env;
use std::time;

// =========== CONFIG ===========
const API_URL: &str = "https://boundvariable.space/communicate";
const COMMAND: &str = "solve spaceship";
const LOAD_DIR: &str = "answers/spaceship";
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

    // scan loading dir
    let files = std::fs::read_dir(LOAD_DIR).unwrap();

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let id = filename.split('.').next().unwrap();
        let id = id.parse::<usize>().unwrap();
        let text = std::fs::read_to_string(path).unwrap();
        eprintln!("submitting id: {}", id);
        let text = format!("{}{} {}", COMMAND, id, text);
        eprintln!("submitting text: {}", text);
        let text = deconvert_string(text);
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
                let tokens = Tokenizer::new(&body).tokenize();
                let node = Parser::new(&tokens).parse();
                let result = Evaluator::new(node).evaluate();
                let text = match result {
                    Node::String(text) => text,
                    _ => panic!("Result is not a string"),
                };
                println!("{}", text);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }

        // 3秒待つ
        tokio::time::sleep(time::Duration::from_secs(3)).await;
    }
}
