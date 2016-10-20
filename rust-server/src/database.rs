use super::models::*;
use super::json_models;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel;

pub fn create_article<'a>(conn: &PgConnection, new_article: NewArticle)  {
    use schema::news;

    diesel::insert(&new_article).into(news::table)
        .get_result::<Article>(conn)
        .expect("Error inserting new article");
} 

pub fn get_articles(conn: &PgConnection, cnt: i64) -> Vec<json_models::Article> {
    use schema::news::dsl::*;

    news.limit(cnt)
        .load::<Article>(conn)
        .expect("Error loading articles")
        .into_iter()
        .map(|a| json_models::Article::from(a))
        .collect()
}

pub fn edit_article(conn: &PgConnection, to_edit: json_models::EditArticle) {
    use schema::news::dsl::*;

    if let Some(new_title) = to_edit.title {
        diesel::update(news.find(to_edit.id))
            .set(title.eq(new_title))
            .execute(conn);
    }

    if let Some(new_text) = to_edit.text {
        diesel::update(news.find(to_edit.id))
            .set(text.eq(new_text))
            .execute(conn);
    }
}

pub fn delete_article(conn: &PgConnection, to_delete: json_models::DeleteArticle) {
    use schema::news::dsl::*;

    diesel::delete(news.find(to_delete.id))
        .execute(conn);
}