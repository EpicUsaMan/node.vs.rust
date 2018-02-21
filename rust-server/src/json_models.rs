use chrono::{DateTime, Utc};
use super::models;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewArticle {
    pub title: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditArticle {
    pub id: i32,
    pub title: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteArticle {
    pub id: i32,
}

impl From<models::Article> for Article {
    #[inline]
    fn from(a: models::Article) -> Article {
        Article {
            id: a.id,
            title: a.title,
            text: a.text,
            created_at: a.createdAt,
            updated_at: a.updatedAt,
        }
    }
}
