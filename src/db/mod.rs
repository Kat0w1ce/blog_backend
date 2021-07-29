pub mod models;
pub mod schema;
use self::models::{Newpost, Post};
// use self::models::*;
use diesel::prelude::*;
// use diesel::query_builder::Query;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub async fn create_post<'a>(
    conn: &SqliteConnection,
    title: &'a str,
    path: &'a str,
    intro: &'a str,
) {
    use schema::posts;
    let newpost = Newpost { title, path, intro };
    diesel::insert_into(posts::table)
        .values(&newpost)
        .execute(conn)
        .expect("ERROR saving new post");
}

pub async fn get_list<'a>(conn: &SqliteConnection) -> Vec<Post> {
    use schema::posts::dsl::*;
    posts.load::<Post>(conn).expect("error loading post lists")
}
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn it_works() {
        let conn = establish_connection();
        // create_post(&conn, "go", "go", "go");
        // create_post(&conn, "rust", "rust", "rust");

        let res = get_list(&conn).await;
        let s = serde_json::to_string(&res).unwrap_or(String::from("none"));
        println!("{}", s);
    }
}
