#[macro_use]
extern crate rocket;

mod api;
mod article;
mod config;
mod db;
mod rss;

use simple_logger::SimpleLogger;

#[launch]
async fn rocket() -> _ {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    api::api()
}
