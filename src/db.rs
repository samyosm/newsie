use chrono::NaiveDate;
use futures::StreamExt;
use mongodb::{
    options::{ClientOptions, IndexOptions, InsertManyOptions},
    results::InsertManyResult,
    Client, Collection, IndexModel,
};

use bson::doc;

use crate::article::Article;

async fn get_collection() -> Collection<Article> {
    let client_options = ClientOptions::parse("mongodb://mongo:27017")
        .await
        .expect("error: couldn't create mongodb options");
    let client = Client::with_options(client_options).expect("error: couldn't connect to mongodb");
    let collection = client
        .database("articles")
        .collection::<Article>("articles");

    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"guid": 1})
        .options(options)
        .build();

    collection
        .create_index(model, None)
        .await
        .expect("error creating index!");

    collection
}

pub async fn store(articles: Vec<Article>) -> anyhow::Result<()> {
    let collection = get_collection().await;

    let options = InsertManyOptions::builder().ordered(false).build();
    collection.insert_many(articles, Some(options)).await?;

    Ok(())
}

// TODO: Support for a single category
pub async fn fetch(date: NaiveDate) -> Vec<Article> {
    let collection = get_collection().await;
    let date = date.to_string();

    collection
        .find(doc! {"date": date}, None)
        .await
        .expect("an error occured while trying to find documents")
        .map(|b| b.expect("an error occured while trying to find documents"))
        .collect::<Vec<Article>>()
        .await
}
