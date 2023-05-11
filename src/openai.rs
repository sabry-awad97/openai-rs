use crate::chat::{self, ChatCompletionRequest, ChatError};
use chat::ChatCompletionResponse;
use log::debug;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use std::fmt::Debug;

pub struct OpenAIClient {
    api_host: String,
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_host: &str, api_key: &str) -> Self {
        OpenAIClient {
            api_host: api_host.to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_request(&self, request: ChatCompletionRequest) -> Result<String, ChatError> {
        let url = format!("{}/v1/chat/completions", self.api_host);
    
        debug!("Sending request to URL: {}", url);
        debug!("Request body: {:?}", request);
    
        let mut retries = 0;
        loop {
            let response = self
                .client
                .post(url.clone())
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .bearer_auth(&self.api_key)
                .json(&request)
                .send()
                .await;
    
            match response {
                Ok(response) => {
                    let status = response.status();
                    let response = response.json::<ChatCompletionResponse>().await?;
    
                    debug!("Response status: {}", status);
    
                    if status.is_success() {
                        log::info!("Response: {:?}", response);
    
                        let result = response
                            .choices
                            .first()
                            .map(|choice| choice.message.content.clone())
                            .ok_or_else(|| ChatError::NoMessageReturned)?;
    
                        debug!("Result: {}", result);
                        return Ok(result);
                    } else {
                        log::error!("Error: {} - {:?}", status, response);
                        return Err(ChatError::ResponseError(status, response));
                    }
                }
                Err(err) => {
                    retries += 1;
                    if retries > 3 {
                        return Err(ChatError::NetworkError(err));
                    }
                    debug!("Error: {}", err);
                    debug!("Retrying request (attempt {})", retries);
                }
            }
        }
    }
}

impl Debug for OpenAIClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIClient")
            .field("api_host", &self.api_host)
            .field(
                "api_key",
                &"sk-**************************************************",
            )
            .finish()
    }
}
