#![allow(non_snake_case)]
use super::schema::*;
use json_models;
use chrono::DateTime;
use chrono::Utc;

#[derive(Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "news"]
pub struct NewArticle {
    pub title: String,
    pub text: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

impl From<json_models::NewArticle> for NewArticle {
    fn from(article: json_models::NewArticle) -> Self {
        let time = Utc::now();
        NewArticle {
            title: article.title,
            text: article.text,
            createdAt: time,
            updatedAt: time
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "news"]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub text: Option<String>,
    pub updatedAt: DateTime<Utc>,
}
