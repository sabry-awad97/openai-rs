use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatCompletionRequest {
    pub model: OAIModel,
    pub messages: Vec<ChatCompletionMessage>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<u64>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatCompletionMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OAIModel {
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3Turbo,
    #[serde(rename = "gpt-4")]
    GPT4,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChoice {
    pub index: u64,
    pub message: ChatCompletionMessage,
    pub finish_reason: Option<String>,
}
