use chrono::DateTime;
use extrablatt::{date::Date, Article as ArticleExtractor};
use futures::{future, stream, StreamExt};
use reqwest::{Client, IntoUrl};
use rss::Channel;

use crate::article::{Article, Category, Language};

// TODO: Possibility add closure to customize channel construction
pub async fn fetch_feeds<T: IntoUrl>(feeds: Vec<T>) -> Vec<Channel> {
    let client = Client::new();

    future::join_all(feeds.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client
                .get(url)
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
        .map(|item| {
            let source = channel.title.clone();
            let language = language.clone();
            let category = category.clone();
            let pub_date = item.pub_date.clone();
            tokio::spawn(async move {
                println!("EXTRACTING: {}", item.clone().title.unwrap());
                let resp = ArticleExtractor::get(item.link().unwrap())
                    .await
                    .expect("error: couldn't extract article")
                    .content;

                Article {
                    title: item.title.unwrap_or(
                        resp.title
                            .expect("error: couldn't get article title")
                            .to_string(),
                    ),
                    content: resp
                        .text
                        .expect("error: couldn't get article content")
                        .to_string(),
                    preview: item.description.unwrap_or(
                        resp.description
                            .expect("error: couldn't get article preview")
                            .to_string(),
                    ),
                    date: if let Some(pub_date) = pub_date {
                        Some(
                            DateTime::parse_from_rfc2822(pub_date.as_str())
                                .unwrap()
                                .date_naive(),
                        )
                    } else if let Some(pub_date) = resp.publishing_date {
                        Some(match pub_date.published {
                            Date::Date(date) => date,
                            Date::DateTime(date_time) => date_time.date(),
                        })
                    } else {
                        None
                    },
                    url: item.link.unwrap(),
                    source,
                    category,
                    language,
                }
            })
        })
        // TODO: Make into a constant
        .buffer_unordered(10);

    return bodies
        .map(|b| b.expect("an error occurred"))
        .collect::<Vec<_>>()
        .await;
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
