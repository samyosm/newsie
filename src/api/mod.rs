mod index;
mod news;
mod populate;

use rocket::{Build, Rocket};

pub fn api() -> Rocket<Build> {
    rocket::build().mount("/", routes![index::index]).mount(
        "/api/v1/",
        routes![index::index, news::news, populate::populate],
    )
}
