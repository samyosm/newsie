use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Article {
    pub guid: String,
    pub title: String,
    pub desc: String,
    pub content: String,
    pub url: String,
    pub source: String,
    pub category: Category,
    pub language: Language,
    pub date: NaiveDate,
}

impl Responder for Article {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Category {
    #[default]
    Tech,
    General,
    Health,
    Culture,
    Sports,
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Language {
    #[default]
    English,
    French,
}
