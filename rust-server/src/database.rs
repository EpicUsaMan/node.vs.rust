use std::env;
use dotenv::dotenv;

use super::models::*;
use super::json_models;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel;

use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    db_pool: DbPool,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db_pool: {
                dotenv().ok();
                let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let manager = ConnectionManager::<PgConnection>::new(db_url);
                Pool::builder()
                    .max_size(64)
                    .build(manager)
                    .expect("Failed to create a pool")
            },
        }
    }

    pub fn create_article(&self, new_article: NewArticle) {
        use schema::news;

        let conn = self.db_pool.get().unwrap();

        diesel::insert_into(news::table)
            .values(&new_article)
            .execute(&*conn)
            .expect("Error inserting new article");
    }

    pub fn get_articles(&self, cnt: i64) -> Vec<json_models::Article> {
        use schema::news::dsl::*;
        let conn = self.db_pool.get().unwrap();
        news.limit(cnt)
            .load::<Article>(&*conn)
            .expect("Error loading articles")
            .into_iter()
            .map(|a| json_models::Article::from(a))
            .collect()
    }

    pub fn edit_article(&self, to_edit: json_models::EditArticle) {
        use schema::news::dsl::*;
        let conn = self.db_pool.get().unwrap();
        diesel::update(news.find(to_edit.id))
            .set(&UpdateArticle {
                title: to_edit.title,
                text: to_edit.text,
            })
            .execute(&*conn)
            .unwrap();
    }

    pub fn delete_article(&self, to_delete: json_models::DeleteArticle) {
        use schema::news::dsl::*;
        let conn = self.db_pool.get().unwrap();
        diesel::delete(news.find(to_delete.id))
            .execute(&*conn)
            .unwrap();
    }
}
