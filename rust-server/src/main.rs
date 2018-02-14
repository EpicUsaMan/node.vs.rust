#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate num_cpus;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate serde;
extern crate serde_json as json;

pub mod schema;
pub mod models;
mod database;
mod json_models;

use rocket::State;
use rocket_contrib::Json;

use json_models::*;
use database::Database;

#[get("/news")]
#[inline]
fn news_get(db: State<Database>) -> Json<Vec<Article>> {
    use std::i64;
    Json(db.get_articles(i64::MAX))
}

#[get("/news/<cnt>")]
#[inline]
fn news_get_cnt(db: State<Database>, cnt: i64) -> Json<Vec<Article>> {
    Json(db.get_articles(cnt))
}

#[post("/news", data = "<input>")]
#[inline]
fn news_post(db: State<Database>, input: Json<EditArticle>) -> &'static str {
    db.edit_article(input.into_inner());
    "{}"
}

#[put("/news", data = "<input>")]
#[inline]
fn news_put(db: State<Database>, input: Json<models::NewArticle>) -> &'static str {
    db.create_article(input.into_inner());
    "{}"
}

#[delete("/news", data = "<input>")]
#[inline]
fn news_delete(db: State<Database>, input: Json<DeleteArticle>) -> &'static str {
    db.delete_article(input.into_inner());
    "{}"
}

fn main() {
    use rocket::config::LoggingLevel;

    let mut config = rocket::Config::production().unwrap();
    config.set_address("localhost").unwrap();
    config.set_port(8000);
    config.set_workers(num_cpus::get() as u16);
    config.set_log_level(LoggingLevel::Critical);

    rocket::custom(config, false)
        .mount("/", routes![news_get, news_get_cnt, news_post, news_put])
        .manage(Database::new())
        .launch();
}
