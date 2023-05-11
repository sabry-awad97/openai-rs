mod chat;
use chat::{
    ChatCompletionMessage, ChatCompletionRequest, ChatCompletionResponse, MessageRole, OAIModel,
};
use log::{error, info};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

pub async fn run(api_host: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
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
    let url = api_host.to_string() + "/v1/chat/completions";

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
    let response = response.json::<ChatCompletionResponse>().await?;

    if status.is_success() {
        info!("Response: {:?}", response);
    } else {
        error!("Error: {} - {:?}", status, response);
    }

    let result = response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| "No message returned")?;

    Ok(result)
}
