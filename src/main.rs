use dotenv::dotenv;
use envconfig::Envconfig;

use openai_rs::{
    self, ChatCompletionMessage, ChatCompletionRequest, MessageRole, OAIModel, OpenAIClient,
};

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
    let client = OpenAIClient::new(&config.api_host, &config.openai_api_key);

    let request = ChatCompletionRequest::builder(
        OAIModel::GPT3Turbo,
        vec![ChatCompletionMessage {
            role: MessageRole::User,
            content: "Hello, GPT-3!".to_string(),
        }],
    )
    .temperature(0.5)
    .top_p(1.0)
    .build()?;

    let response = client.send_request(request).await?;
    println!("{}", response);

    Ok(())
}
