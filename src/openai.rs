use crate::chat::{self, ChatCompletionRequest, ChatError};
use chat::ChatCompletionResponse;
use log::{debug, error, info};
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    ClientBuilder, StatusCode, Url,
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
                    debug!("Response status: {}", status);

                    match status {
                        StatusCode::OK => {
                            let response_text = response.text().await?;
                            let response_json =
                                serde_json::from_str::<ChatCompletionResponse>(&response_text)
                                    .map_err(|err| {
                                        ChatError::InvalidResponseFormat(err.to_string())
                                    })?;

                            info!("Request successful {:#?}", response_json);
                            let result = response_json
                                .choices
                                .first()
                                .map(|choice| choice.message.content.clone())
                                .ok_or_else(|| ChatError::NoMessageReturned)?;
                            debug!("Result: {}", result);
                            return Ok(result);
                        }

                        status_code => {
                            error!("Response status: {}", status_code);
                            return Err(ChatError::ResponseError(status_code, response));
                        }
                    };
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
