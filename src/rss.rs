use extrablatt::Article as ArticleExtractor;
use futures::{future, stream, StreamExt};
use reqwest::{Client, IntoUrl};
use rss::Channel;

use crate::article::{Article, Category};

// TODO: Possibility add closure to customize channel construction
async fn fetch_feeds<T: IntoUrl>(feeds: Vec<T>) -> Vec<Channel> {
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

async fn fetch_channel(channel: Channel, category: Category) -> Vec<Article> {
    let bodies = stream::iter(channel.items.clone())
        .map(|item| {
            let source = channel.title.clone();
            let language = channel.language.clone();
            let category = category.clone();
            tokio::spawn(async move {
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
                    url: item.link.unwrap(),
                    source,
                    category,
                    language: language.unwrap_or(
                        resp.language
                            .expect("error: couldn't get article language")
                            .full_name()
                            .to_string(),
                    ),
                }
            })
        })
        .buffer_unordered(10);
    return bodies
        .map(|b| b.expect("an error occurred"))
        .collect::<Vec<_>>()
        .await;
}
