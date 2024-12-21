use bsky_sdk::{
    agent::config::{Config, FileStore},
    BskyAgent,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = BskyAgent::builder()
        .config(Config::load(&FileStore::new("config.json")).await?)
        .build()
        .await?;
    let result = agent.api.com.atproto.server.get_session().await;
    assert!(result.is_ok());
    Ok(())
}
