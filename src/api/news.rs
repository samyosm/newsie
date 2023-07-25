use chrono::NaiveDate;
use rocket::serde::json::Json;

use crate::{
    article::{Article, Category, Language},
    db::fetch,
};

#[get("/news?<date>&<category>&<language>")]
pub async fn news(
    date: &str,
    category: Option<Category>,
    language: Option<Language>,
) -> Json<Vec<Article>> {
    let date =
        NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("error: expected %Y-%m-%d date format");
    let articles = fetch(date, category, language).await;
    Json(articles)
}
