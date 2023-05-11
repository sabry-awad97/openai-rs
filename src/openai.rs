use crate::chat::{self, ChatCompletionRequest, ChatError};
use chat::ChatCompletionResponse;
use log::debug;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    ClientBuilder, Url,
};
use std::fmt::Debug;

pub struct OpenAIClient {
    api_host: Url,
    api_key: String,
    client: reqwest::Client,
    max_retries: u8,
}

impl OpenAIClient {
    pub fn new(api_host: &str, api_key: &str) -> Result<Self, ChatError> {
        let api_host =
            Url::parse(api_host).map_err(|err| ChatError::InvalidUrl(err.to_string()))?;
        let client = ClientBuilder::new().build()?;
        Ok(OpenAIClient {
            api_host,
            api_key: api_key.to_string(),
            client,
            max_retries: 3,
        })
    }

    pub async fn send_request(&self, request: ChatCompletionRequest) -> Result<String, ChatError> {
        let url = self
            .api_host
            .join("/v1/chat/completions")
            .map_err(|err| ChatError::InvalidUrl(err.to_string()))?;

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
                    if retries > self.max_retries {
                        return Err(ChatError::NetworkError(err));
                    }
                    debug!("Error: {}", err);
                    debug!(
                        "Retrying request (attempt {}/{})",
                        retries, self.max_retries
                    );
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
