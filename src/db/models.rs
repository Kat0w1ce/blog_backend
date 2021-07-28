use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub intro: Option<String>,
    pub date: String,
}

use super::schema::posts;
#[derive(Insertable)]
#[table_name = "posts"]
pub struct Newpost<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub intro: &'a str,
}
