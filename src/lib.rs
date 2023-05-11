mod chat;
pub use chat::{ChatCompletionMessage, ChatCompletionRequest, MessageRole, OAIModel};
mod openai;
pub use openai::OpenAIClient;
