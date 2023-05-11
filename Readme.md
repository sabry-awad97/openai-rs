# OpenAi Rust

This is a Rust crate that provides an OpenAI client for chat completions. It allows you to easily send requests to the OpenAI API and receive completion suggestions for chat messages.

## Installation

To use this crate, add the following to your `Cargo.toml` file:

```toml
[dependencies]
openai_rs = { git = "https://github.com/sabry-awad97/openai-rs" }
```

## Usage

First, create a new `OpenAIClient` instance with your API host and key:

```rs
use mod_chat::OpenAIClient;

let client = OpenAIClient::new("https://api.openai.com", "sk-***********");
```

Then, send a request to the API with a `ChatCompletionRequest`:

```rs
use openai_rs::{ChatCompletionMessage, ChatCompletionRequest, MessageRole, OAIModel};

let request = ChatCompletionRequest::builder(
        OAIModel::GPT3Turbo,
        vec![ChatCompletionMessage {
            role: MessageRole::User,
            content: "Hello, how are you?".to_string(),
        }],
    )
    .temperature(0.5)
    .top_p(1.0)
    .build()?;

let message = client.send_request(request).await?;
```

The `send_request` method returns a `Result<String, ChatError>`, where the `String` contains the completed message and `ChatError` is an enum that represents errors that may occur during the request.

## Example

Here's a complete example that sends a chat message and prints the response:

```rs
use dotenv::dotenv;
use envconfig::Envconfig;

use openai_rs::{ChatCompletionMessage, ChatCompletionRequest, MessageRole, OAIModel, OpenAIClient};

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
            content: "Hello, how are you?".to_string(),
        }],
    )
    .temperature(0.5)
    .top_p(1.0)
    .build()?;

    let message = client.send_request(request).await?;
    println!("{}", message);

    Ok(())
}
```

## License

This crate is licensed under the MIT License.
