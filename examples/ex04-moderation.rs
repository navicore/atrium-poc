use bsky_sdk::{
    agent::config::{Config, FileStore},
    moderation::decision::DecisionContext,
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

    // First get the user's moderation prefs and their label definitions
    let preferences = agent.get_preferences(true).await?;
    let moderator = agent.moderator(&preferences).await?;

    // in feeds
    let output = agent
        .api
        .app
        .bsky
        .feed
        .get_timeline(
            atrium_api::app::bsky::feed::get_timeline::ParametersData {
                algorithm: None,
                cursor: None,
                limit: None,
            }
            .into(),
        )
        .await?;
    for feed_view_post in &output.feed {
        // We call the appropriate moderation function for the content
        let post_mod = moderator.moderate_post(&feed_view_post.post);
        // don't include in feeds?
        println!(
            "{:?} (filter: {})",
            feed_view_post.post.cid.as_ref(),
            post_mod.ui(DecisionContext::ContentList).filter()
        );
    }
    Ok(())
}
