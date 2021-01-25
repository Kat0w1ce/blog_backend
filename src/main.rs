mod api;
use self::api::*;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(index).service(blog)
    })
    .bind("0.0.0.0:9999")?
    .run()
    .await
}
