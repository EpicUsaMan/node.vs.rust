use super::models;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: i32,
    pub time: i64,
    pub title: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditArticle {
    pub id: i32,
    pub title: Option<String>,
    pub text: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteArticle {
    pub id: i32
}

impl From<models::Article> for Article {
    fn from(a: models::Article) -> Article {
        Article {
            id: a.id,
            time: a.time.timestamp(),
            title: a.title,
            text: a.text
        }
    }
}