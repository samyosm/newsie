use chrono::DateTime;
use extrablatt::{date::Date, Article as ArticleExtractor};
use futures::{future, stream, StreamExt};
use reqwest::Client;
use rss::Channel;

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
    channel: &Channel,
    category: Category,
    language: Language,
) -> Vec<Article> {
    let bodies = stream::iter(channel.items.clone())
        .map(|item| -> tokio::task::JoinHandle<Option<Article>> {
            let source = channel.title.clone();
            let language = language.clone();
            let category = category.clone();
            let pub_date = item.pub_date.clone();

            tokio::spawn(async move {
                println!("EXTRACTING: {}", item.title.clone().unwrap());

                if let Ok(content) = ArticleExtractor::content(item.link().unwrap()).await {
                    Some(Article {
                        guid: if let Some(guid) = item.guid {
                            guid.value
                        } else {
                            return None;
                        },
                        title: if let Some(title) = item.title {
                            title
                        } else if let Some(title) = content.title {
                            title.to_string()
                        } else {
                            return None;
                        },

                        content: if let Some(content) = content.text {
                            content.to_string()
                        } else {
                            return None;
                        },
                        desc: if let Some(preview) = item.description {
                            preview
                        } else if let Some(preview) = content.description {
                            preview.to_string()
                        } else {
                            return None;
                        },
                        date: if let Some(pub_date) = pub_date {
                            DateTime::parse_from_rfc2822(pub_date.as_str())
                                .unwrap()
                                .date_naive()
                        } else if let Some(pub_date) = content.publishing_date {
                            match pub_date.published {
                                Date::Date(date) => date,
                                Date::DateTime(date_time) => date_time.date(),
                            }
                        } else {
                            return None;
                        },
                        url: item.link.unwrap(),
                        source,
                        category,
                        language,
                    })
                } else {
                    None
                }
            })
        })
        // TODO: Make into a constant
        .buffer_unordered(10);

    bodies
        .map(|b| b.expect("an error occurred"))
        .filter_map(|f| async { f })
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
            tokio::spawn(async move { fetch_channel(&channel, category, language).await })
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
