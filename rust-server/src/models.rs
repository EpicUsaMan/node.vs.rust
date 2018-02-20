use super::schema::*;
use chrono::naive::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub time: NaiveDateTime,
    pub title: String,
    pub text: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "news"]
pub struct NewArticle {
    pub title: String,
    pub text: String,
}

#[derive(AsChangeset)]
#[table_name = "news"]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub text: Option<String>,
}
