use crate::rss::{fetch_channels, fetch_feeds};

mod article;
mod db;
mod rss;

use crate::article::{Category, Language};

#[tokio::main]
async fn main() {
    let feeds = vec![
        "https://mashable.com/feeds/rss/all",
        "https://www.france24.com/en/rss",
        "https://www.france24.com/en/france/rss",
        "https://www.wired.com/rss",
    ];

    println!("Fetching Feeds");
    let channels = fetch_feeds(feeds).await;

    let articles = fetch_channels(channels, Category::General, Language::English).await;

    println!("{:?}", articles);
    println!("count: {}", articles.len())
}
