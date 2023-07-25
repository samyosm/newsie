use rocket::http::Status;

use crate::{
    config::Config,
    db::store,
    rss::{collect_channels, scrap_channels},
};

#[get("/populate?<auth>")]
pub async fn populate(auth: &str) -> Status {
    if auth
        != std::env::var("FETCH_AUTHORIZATION").expect("error: no fetch authorization env variable")
    {
        return Status::Unauthorized;
    }

    let feed_configs = Config::read_config()
        .await
        .expect("error: couldn't read config file")
        .config;
    for feed in feed_configs {
        let channels = collect_channels(feed.feeds).await;
        let articles = scrap_channels(channels, feed.category, feed.language).await;
        let _ = store(articles).await;
    }

    Status::Ok
}
