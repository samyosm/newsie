use chrono::NaiveDate;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Article {
    pub title: String,
    pub preview: String,
    pub content: String,
    pub url: String,
    pub source: String,
    pub category: Category,
    // TODO: Make into an enum but where you can parse a string like 'us' into the enum using
    // FromStr
    pub language: Language,
    pub date: Option<NaiveDate>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Category {
    #[default]
    Tech,
    General,
    Health,
    Culture,
    Sports,
}
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Language {
    #[default]
    English,
    French,
}
