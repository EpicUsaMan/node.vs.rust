#![feature(box_syntax)]
#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(proc_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json as json;

extern crate futures_await as futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate tokio_core;

pub mod schema;
pub mod models;
mod database;
mod json_models;

use serde::Serialize;

use std::rc::Rc;

use json_models::*;

use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::Method;

use futures::prelude::*;
use futures::future;

use database::Database;

type HyperResult<T> = Result<T, hyper::Error>;

fn main() {
    let database = Database::new();
    let database = Rc::new(database);

    println!("Connected to database");

    let addr = "127.0.0.1:8080".parse().unwrap();

    /*
    let router = router! (
        get:     get    "/news/"     => get_handler,
        get_cnt: get    "/news/:cnt" => get_handler,
        post:    post   "/news/"     => post_handler,
        put:     put    "/news/"     => put_handler,
        delete:  delete "/news/"     => delete_handler
    );
    */

    let cpupool = futures_cpupool::CpuPool::new_num_cpus();

    let router = service_fn(move |req: Request| {
        let database = database.clone();
        cpupool.spawn(move || {
            match *req.method() {
                Method::Get => get_handler(database, req),
                Method::Post => post_handler(database, req),
                Method::Put => put_handler(database, req),
                Method::Delete => delete_handler(database, req),
                _ => box future::ok(resp_not_found()),
            }
        })
    });

    let router = const_service(router);

    println!("Listening on {}", addr);

    Http::new().bind(&addr, router).unwrap().run().unwrap();
}

#[async(boxed)]
fn get_handler(db: Rc<Database>, req: Request) -> HyperResult<Response> {
    if let Ok(cnt) = parse_get_news(req.path()) {
        let articles: Vec<Article> = await!(db.get_articles(cnt)).unwrap().into_iter().collect();
        Ok(resp_ok(articles))
    } else {
        Ok(resp_not_found())
    }
}

fn parse_get_news(url: &str) -> Result<i64, ()> {
    use std::i64;
    use std::str::FromStr;

    let mut parts = url.split("/");

    if parts.next() != Some("news") {
        return Err(());
    }

    if let Some(cnt) = parts.next() {
        // ensure there is no other parts
        if parts.next().is_none() {
            return Ok(i64::from_str(cnt).map_err(|_| ())?);
        } else {
            return Err(());
        }
    } else {
        return Ok(i64::MAX);
    }
}

#[async(boxed)]
fn post_handler(db: Rc<Database>, req: Request) -> HyperResult<Response> {
    let body = await!(req.body().concat2())?;
    let edit_article: EditArticle = json::from_slice(&body).unwrap();

    await!(db.edit_article(edit_article)).unwrap();

    Ok(resp_ok("{}"))
}

#[async(boxed)]
fn put_handler(db: Rc<Database>, req: Request) -> HyperResult<Response> {
    let body = await!(req.body().concat2())?;
    let new_article: models::NewArticle = json::from_slice(&body).unwrap();

    await!(db.create_article(new_article)).unwrap();

    Ok(resp_ok("{}"))
}

#[async(boxed)]
fn delete_handler(db: Rc<Database>, req: Request) -> HyperResult<Response> {
    let body = await!(req.body().concat2())?;
    let delete_article: DeleteArticle = json::from_slice(&body).unwrap();

    await!(db.delete_article(delete_article)).unwrap();

    Ok(resp_ok("{}"))
}

fn resp_ok<S: Serialize>(body: S) -> Response {
    let mut response = Response::new();
    response.set_status(hyper::Ok);
    response.set_body(json::to_string(&body).unwrap());
    response
}

fn resp_not_found() -> Response {
    let mut response = Response::new();
    response.set_status(hyper::NotFound);
    response
}
