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

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Default,
    FromFormField,
)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    #[default]
    Tech,
    General,
    Health,
    Culture,
    Sports,
}
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Default,
    FromFormField,
)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    English,
    French,
}
