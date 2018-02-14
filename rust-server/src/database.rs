use std::env;
use dotenv::dotenv;

use super::models::*;
use super::json_models;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel;

use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use futures_cpupool::CpuPool;
use futures::Future;

type DbPool = Pool<ConnectionManager<PgConnection>>;
type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Database {
    db_pool: DbPool,
    thread_pool: CpuPool,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db_pool: {
                dotenv().ok();
                let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let manager = ConnectionManager::<PgConnection>::new(db_url);
                Pool::builder().max_size(16).build(manager).expect("Failed to create a pool")
            },
            thread_pool: CpuPool::new(16),
        }
    }

    fn spawn_async<R, F>(&self, f: F) -> impl Future<Item = R, Error = ()>
    where
        R: Send + 'static,
        F: Sync + Send + 'static + FnOnce(Conn) -> R,
    {
        let conn = self.db_pool.get().expect("failed to get pooled connection");
        self.thread_pool.spawn_fn(move || Ok(f(conn)))
    }

    pub fn create_article(&self, new_article: NewArticle) -> impl Future<Item = (), Error = ()> {
        use schema::news;

        self.spawn_async(move |conn| {
            diesel::insert_into(news::table)
                .values(&new_article)
                .execute(&*conn)
                .expect("Error inserting new article");
        })
    }

    pub fn get_articles(
        &self,
        cnt: i64,
    ) -> impl Future<Item = Vec<json_models::Article>, Error = ()> {
        use schema::news::dsl::*;
        self.spawn_async(move |conn| {
            news.limit(cnt)
                .load::<Article>(&*conn)
                .expect("Error loading articles")
                .into_iter()
                .map(|a| json_models::Article::from(a))
                .collect()
        })
    }

    pub fn edit_article(
        &self,
        to_edit: json_models::EditArticle,
    ) -> impl Future<Item = (), Error = ()> {
        use schema::news::dsl::*;

        self.spawn_async(move |conn| {
            diesel::update(news.find(to_edit.id))
                .set(&UpdateArticle {
                    title: to_edit.title,
                    text: to_edit.text,
                })
                .execute(&*conn)
                .unwrap();
        })
    }

    pub fn delete_article(
        &self,
        to_delete: json_models::DeleteArticle,
    ) -> impl Future<Item = (), Error = ()> {
        use schema::news::dsl::*;

        self.spawn_async(move |conn| {
            diesel::delete(news.find(to_delete.id))
                .execute(&*conn)
                .unwrap();
        })
    }
}
