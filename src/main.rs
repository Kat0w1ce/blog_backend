pub mod db;
pub mod handler;
pub mod router;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;

use actix_cors::Cors;
use actix_files;
use actix_web::{web, App, HttpServer, Responder};
use diesel::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use router::get_posts;
// use router::get_list;
async fn index() -> impl Responder {
    // println!("hello");
    "Hello world!"
}

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("can not find database");

    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Error when establishing connection pool");
    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/posts", web::get().to(get_posts))
            .service(actix_files::Files::new("/blog", "./assets").show_files_listing())
    })
    .bind("0.0.0.0:9999")?
    .run()
    .await
}
