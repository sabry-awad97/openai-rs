use dotenv::dotenv;
use envconfig::Envconfig;
use log::{error, info};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct ChatCompletionRequest {
    model: OAIModel,
    messages: Vec<ChatCompletionMessage>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<u64>,
    frequency_penalty: Option<f32>,
    presence_penalty: Option<f32>,
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

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "OPENAI_API_KEY")]
    pub openai_api_key: String,
    #[envconfig(from = "API_HOST", default = "https://api.openai.com")]
    pub api_host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let config = Config::init_from_env()?;

    let request = ChatCompletionRequest {
        model: OAIModel::GPT3Turbo,
        messages: vec![ChatCompletionMessage {
            role: MessageRole::User,
            content: "Hello, GPT-3!".to_string(),
        }],
        temperature: Some(0.5),
        top_p: Some(1.0),
        max_tokens: Some(50),
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
    };

    let client = reqwest::Client::new();
    let url = config.api_host + "/v1/chat/completions";
    let api_key = config.openai_api_key;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let body = response.json::<ChatCompletionResponse>().await?;

    if status.is_success() {
        info!("Response: {:?}", body);
    } else {
        error!("Error: {} - {:?}", status, body);
    }

    Ok(())
}
