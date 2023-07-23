use anyhow::{anyhow, Result};
use article_scraper::ArticleScraper;
use chrono::DateTime;
use futures::{future, stream, StreamExt};
use reqwest::Client;
use rss::{Channel, Guid};
use url::Url;

use crate::article::{Article, Category, Language};

// TODO: Possibility add closure to customize channel construction
// TODO: !! Hash RSS xml and return new ones only
pub async fn fetch_feeds(feeds: Vec<String>) -> Vec<Channel> {
    let client = Client::new();

    future::join_all(feeds.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client
                .get(url.as_str())
                .send()
                .await
                .expect("error: couldn't fetch RSS Feed");
            let bytes = resp
                .bytes()
                .await
                .expect("error: couldn't get bytes from fetched RSS Feed");
            Channel::read_from(&bytes[..]).expect("error: couldn't read fetched RSS Feed")
        }
    }))
    .await
}

pub async fn fetch_channel(
    channel: Channel,
    category: Category,
    language: Language,
) -> Vec<Article> {
    let bodies = stream::iter(channel.items)
        .map(|item| -> tokio::task::JoinHandle<Result<Article>> {
            let client = Client::new();

            let source = channel.title.clone();
            let language = language.clone();
            let category = category.clone();

            tokio::spawn(async move {
                println!("EXTRACTING: {}", item.title.as_ref().unwrap());

                let scraper = ArticleScraper::new(None).await;
                let url = Url::parse(item.link().unwrap())?;
                let article = scraper.parse(&url, false, &client, None).await?;

                if let (
                    Some(Guid { value: guid, .. }),
                    Some(title),
                    Some(desc),
                    Some(content),
                    Some(url),
                    Some(date),
                ) = (
                    item.guid,
                    item.title.or(article.title),
                    item.description,
                    article.html,
                    item.link,
                    item.pub_date,
                ) {
                    Ok(Article {
                        guid,
                        title,
                        desc,
                        content,
                        url,
                        source,
                        category,
                        language,
                        date: DateTime::parse_from_rfc2822(date.as_str())
                            .unwrap()
                            .date_naive(),
                    })
                } else {
                    Err(anyhow!("Not enough information"))
                }
            })
        })
        // TODO: Make into a constant
        .buffer_unordered(10);

    bodies
        .map(|b| b.expect("an error occurred"))
        .filter_map(|f| async { f.ok() })
        .collect::<Vec<_>>()
        .await
}

pub async fn fetch_channels(
    channels: Vec<Channel>,
    category: Category,
    language: Language,
) -> Vec<Article> {
    let bodies = stream::iter(channels)
        .map(|channel| {
            let category = category.clone();
            let language = language.clone();
            tokio::spawn(async move { fetch_channel(channel, category, language).await })
        })
        .buffer_unordered(10);

    bodies
        .map(|b| b.expect("an error occurred"))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect()
}
