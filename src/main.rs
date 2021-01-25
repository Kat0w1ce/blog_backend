mod api;
use self::api::*;
use actix_web::{App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(blog))
        .bind("localhost:9999")?
        .run()
        .await
}
