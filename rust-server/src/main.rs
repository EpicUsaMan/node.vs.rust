#![feature(proc_macro)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(box_syntax)]
#[macro_use] extern crate router;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
extern crate chrono;
extern crate dotenv;
extern crate iron;
extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod schema;
pub mod models;
mod database;
mod json_models;

use diesel::connection::Connection;
use diesel::pg::*;
use r2d2::{ManageConnection, Pool};
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

use iron::prelude::*;
use iron::status;
use router::Router;
use router::Params;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};

use json_models::*;

lazy_static! {    
    static ref POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create a pool");
        pool
    };
}

fn main() {
    let router = router! (
        get:     get    "/news/"     => getHandler,
        get_cnt: get    "/news/:cnt" => getHandler,
        post:    post   "/news/"     => postHandler,
        put:     put    "/news/"     => putHandler,
        delete:  delete "/news/"     => deleteHandler
    );

    Iron::new(router).http("localhost:8080").unwrap();
}

fn getHandler(r: &mut Request) -> IronResult<Response> {
    use std::i64;
    use std::str::FromStr;
    
    let conn = POOL.get().unwrap();
    let cnt = r.extensions.get::<Router>().unwrap()
        .find("cnt")
        .map(|s| i64::from_str(s).unwrap())
        .unwrap_or(i64::MAX);

    let articles: Vec<Article> = database::get_articles(&conn, cnt)
        .into_iter()
        .map(|a| a.into())
        .collect();

    response(status::Ok, Some(box json::encode(&articles).unwrap()))
}

fn postHandler(r: &mut Request) -> IronResult<Response> {
    let conn = POOL.get().unwrap();

    let mut s = String::with_capacity(512);
    r.body.read_to_string(&mut s);

    let edit_article: EditArticle = json::decode(&s).unwrap();

    database::edit_article(&conn, edit_article);

    response(status::Ok, Some(box "{}"))
}

fn putHandler(r: &mut Request) -> IronResult<Response> {
    let conn = POOL.get().unwrap();

    let mut s = String::with_capacity(512);
    r.body.read_to_string(&mut s);

    let new_article: models::NewArticle = json::decode(&s).unwrap();

    database::create_article(&conn, new_article);

    response(status::Ok, Some(box "{}"))
}

fn deleteHandler(r: &mut Request) -> IronResult<Response> {
    let conn = POOL.get().unwrap();

    let mut s = String::with_capacity(16);
    r.body.read_to_string(&mut s);

    let delete_article: DeleteArticle = json::decode(&s).unwrap();

    database::delete_article(&conn, delete_article);
    
    response(status::Ok, Some(box "{}"))
}

fn response(s: status::Status, body: Option<Box<iron::response::WriteBody + 'static>>) -> IronResult<Response> {
    let mut response = Response::with(s);
    response.body = body;
    Ok(response)
}
