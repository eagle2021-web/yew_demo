use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionData {
    pub     id: String,
    pub    object: String,
    pub    model: String,
    pub   created: u64,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub  index: i32,
    pub delta: Delta,
    pub  finish_reason: Option<String>, // 注意在Rust中，命名习惯是使用蛇形命名法（snake_case）
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    pub content: String,
}