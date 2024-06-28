use dotenv::dotenv;
use reqwest;
use std::env;

use serde::{Deserialize, Serialize};

const API_URL: &str = "https://boundvariable.space/communicate";
struct Env {
    token: String,
}

fn main() {
    dotenv().ok();

    let env = Env {
        token: env::var("TOKEN").expect("TOKEN must be set"),
    };

    let client = reqwest::Client::new();

    let response = client
        .post(API_URL)
        .header("Authorization", format!("Bearer {}", env.token))
        .header("Content-Type", "application/json");

    println!("Hello! I'm a binary.");
}
