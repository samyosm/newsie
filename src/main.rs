use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header::ContentType;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    db::{fetch, store},
    rss::{fetch_channels, fetch_feeds},
};

use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use batch::get_batches;

mod article;
mod batch;
mod db;
mod rss;

async fn save_batches_from_dir() {
    for batch in get_batches() {
        let channels = fetch_feeds(batch.feeds).await;

        let articles = fetch_channels(channels, batch.category, batch.language).await;
        let _ = store(articles).await;
    }
}

#[derive(Deserialize)]
struct Query {
    authorization: String,
}

#[post("/api/v1/fetch")]
async fn update_db(params: Option<web::Query<Query>>) -> impl Responder {
    match params {
        Some(params)
            if params.authorization
                == std::env::var("FETCH_AUTHORIZATION")
                    .expect("error: no fetch authorization env") =>
        {
            save_batches_from_dir().await;
            HttpResponse::Ok().body("SUCCESSFUL")
        }
        _ => HttpResponse::MethodNotAllowed()
            .status(http::StatusCode::UNAUTHORIZED)
            .body("UNAUTHORIZED"),
    }
}

#[get("/api/v1/news/{date}")]
async fn news(path: web::Path<NaiveDate>) -> impl Responder {
    let date = path.into_inner();

    println!("REQUEST: {}", date);
    let articles = fetch(date).await;
    let body = serde_json::to_string(&articles).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting!");
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(10)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&governor_conf))
            .service(update_db)
            .service(news)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
