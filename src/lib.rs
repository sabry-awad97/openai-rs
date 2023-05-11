mod chat;
use crate::chat::ChatError;
use chat::ChatCompletionResponse;
pub use chat::{ChatCompletionMessage, ChatCompletionRequest, MessageRole, OAIModel};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
pub struct ChatClient {
    api_host: String,
    api_key: String,
    client: reqwest::Client,
}

impl ChatClient {
    pub fn new(api_host: &str, api_key: &str) -> Self {
        ChatClient {
            api_host: api_host.to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn run(&self, request: ChatCompletionRequest) -> Result<String, ChatError> {
        let url = format!("{}/v1/chat/completions", self.api_host);

        let response = self
            .client
            .post(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let response = response.json::<ChatCompletionResponse>().await?;

        if status.is_success() {
            log::info!("Response: {:?}", response);
        } else {
            log::error!("Error: {} - {:?}", status, response);
            return Err(ChatError::ResponseError(status, response));
        }

        let result = response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| ChatError::NoMessageReturned)?;

        Ok(result)
    }
}
