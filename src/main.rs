use dotenv::dotenv;
use envconfig::Envconfig;

use openai_rs;

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
    match openai_rs::run(&config.api_host, &config.openai_api_key).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
    Ok(())
}
