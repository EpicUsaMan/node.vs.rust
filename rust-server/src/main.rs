#![feature(proc_macro)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(box_syntax)]
#[macro_use] extern crate router;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate chrono;
extern crate dotenv;
extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod schema;
pub mod models;
mod database;
mod json_models;

use diesel::pg::*;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

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
        get:     get    "/news/"     => get_handler,
        get_cnt: get    "/news/:cnt" => get_handler,
        post:    post   "/news/"     => post_handler,
        put:     put    "/news/"     => put_handler,
        delete:  delete "/news/"     => delete_handler
    );

    Iron::new(router).http("localhost:8080").unwrap();
}

fn get_handler(r: &mut Request) -> IronResult<Response> {
    use std::i64;
    use std::str::FromStr;
    
    let conn = get_db_connection();
    let cnt = r.extensions.get::<Router>().unwrap()
        .find("cnt")
        .map(|s| i64::from_str(s).unwrap())
        .unwrap_or(i64::MAX);

    let articles: Vec<Article> = database::get_articles(&conn, cnt)
        .into_iter()
        .map(|a| a.into())
        .collect();

    response(status::Ok, Some(box json::to_string(&articles).unwrap()))
}

fn post_handler(r: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();

    let mut s = String::with_capacity(512);
    r.body.read_to_string(&mut s).unwrap();

    let edit_article: EditArticle = json::from_str(&s).unwrap();

    database::edit_article(&conn, edit_article);

    response(status::Ok, Some(box "{}"))
}

fn put_handler(r: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();

    let mut s = String::with_capacity(512);
    r.body.read_to_string(&mut s).unwrap();

    let new_article: models::NewArticle = json::from_str(&s).unwrap();

    database::create_article(&conn, new_article);

    response(status::Ok, Some(box "{}"))
}

fn delete_handler(r: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();

    let mut s = String::with_capacity(16);
    r.body.read_to_string(&mut s).unwrap();

    let delete_article: DeleteArticle = json::from_str(&s).unwrap();

    database::delete_article(&conn, delete_article);
    
    response(status::Ok, Some(box "{}"))
}

fn response(s: status::Status, body: Option<Box<iron::response::WriteBody + 'static>>) -> IronResult<Response> {
    let mut response = Response::with(s);
    response.body = body;
    Ok(response)
}

fn get_db_connection() -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    loop {
        match POOL.get() {
            Err(r2d2::GetTimeout(_)) => std::thread::sleep(std::time::Duration::from_millis(10)),
            Ok(conn) => return conn,
        }
    }
} 