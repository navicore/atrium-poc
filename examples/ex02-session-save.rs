use bsky_sdk::{agent::config::FileStore, BskyAgent};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file
    let email = env::var("BSKY_EMAIL")?;
    let password = env::var("BSKY_PASSWORD")?;

    let agent = BskyAgent::builder().build().await?;
    let _session = agent.login(&email, &password).await?;
    agent
        .to_config()
        .await
        .save(&FileStore::new("config.json"))
        .await?;
    Ok(())
}
