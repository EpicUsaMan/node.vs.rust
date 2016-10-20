use super::schema::*;
use chrono::naive::datetime::NaiveDateTime;
use rustc_serialize::{Encodable, Decodable};
use rustc_serialize::{Encoder, Decoder};

#[derive(Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub time: NaiveDateTime,
    pub title: String,
    pub text: String,
}

#[derive(Insertable, RustcEncodable, RustcDecodable, Debug)]
#[table_name="news"]
pub struct NewArticle {
    pub title: String,
    pub text: String,
}