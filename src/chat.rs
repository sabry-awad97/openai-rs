use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(strip_option, into))]
pub struct ChatCompletionRequest {
    model: ChatCompletionModel,
    messages: Vec<ChatCompletionMessage>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
}

impl ChatCompletionRequest {
    pub fn builder(
        model: impl Into<ChatCompletionModel>,
        messages: impl Into<Vec<ChatCompletionMessage>>,
    ) -> ChatCompletionRequestBuilder {
        ChatCompletionRequestBuilder::create_empty()
            .model(model)
            .messages(messages)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatCompletionMessage {
    pub role: ChatCompletionMessageRole,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionMessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatCompletionModel {
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
    pub usage: Option<ChatCompletionUsage>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionUsage {
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

#[derive(Debug)]
pub enum ChatError {
    InvalidUrl(String),
    NetworkError(reqwest::Error),
    ResponseError(reqwest::StatusCode, reqwest::Response),
    NoMessageReturned,
    InvalidResponseFormat(String),
}

impl std::error::Error for ChatError {}

impl std::fmt::Display for ChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatError::InvalidUrl(e) => write!(f, "URL parse error: {}", e),
            ChatError::NetworkError(e) => write!(f, "Request error: {}", e),
            ChatError::ResponseError(status, response) => {
                write!(
                    f,
                    "Unexpected response: status: {} - {:?}",
                    status, response
                )
            }
            ChatError::NoMessageReturned => write!(f, "No message returned"),
            ChatError::InvalidResponseFormat(msg) => {
                write!(f, "Invalid response format: {}", msg)
            }
        }
    }
}

impl From<reqwest::Error> for ChatError {
    fn from(error: reqwest::Error) -> Self {
        ChatError::NetworkError(error)
    }
}
