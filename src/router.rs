pub use crate::db::models::Post;
use crate::db::{self, get_list};
use crate::Pool;
use actix_multipart::Multipart;
use actix_web::{http, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::fs;
use std::io::Write;
pub async fn get_posts(pool: web::Data<Pool>) -> impl Responder {
    // let posts=g
    let conn = pool.get().expect("Error when getting conn from pool");
    let res = get_list(&conn).await;
    let serialized = serde_json::to_string(&res).unwrap();
    // println!("{}", serialized);
    HttpResponse::build(http::StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Method", "get")
        .content_type("application/json")
        .body(serialized)
}

#[derive(Deserialize)]
pub struct FileInfo {
    filename: String,
    title: String,
    path: String,
    type_: String,
}

pub async fn post(
    pool: web::Data<Pool>,
    query: web::Query<FileInfo>,
    mut payload: Multipart,
) -> impl Responder {
    println!("start upload");
    let url = format!("assets/{}/{}", query.path, query.filename);
    // let url = format!("assets/{}", query.filename);
    println!("{}", url);
    // let savefile= |url| std::fs::File::create(url)
    while let Ok(Some(mut field)) = payload.try_next().await {
        let path = format!("./assets/{}", query.path);
        let url = url.clone();
        let mut f = web::block(|| {
            std::fs::create_dir_all(path).unwrap();
            std::fs::File::create(url)
        })
        .await
        .unwrap();
        println!("file created ");
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap();
        }
    }
    if query.type_ == String::from(".html") {
        let conn = pool.get().expect("Error when getting conn from pool");
        use db::create_post;
        let filepath = format!("{}/{}", query.path, query.filename);
        create_post(&conn, &query.title, &filepath, "").await;
        // "ok"
    }
    println!("uploaded");
    HttpResponse::Ok()
        .content_type("multipart/form-data")
        .body("ok")
}
