use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionData {
    id: String,
    object: String,
    model: String,
    created: u64,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: i32,
    delta: Delta,
    finish_reason: Option<String>, // 注意在Rust中，命名习惯是使用蛇形命名法（snake_case）
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    content: String,
}