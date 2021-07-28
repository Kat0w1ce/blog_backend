use crate::db::get_list;
pub use crate::db::models::Post;
use crate::Pool;
use actix_web::{http, web, HttpResponse, Responder};
pub async fn get_posts(pool: web::Data<Pool>) -> impl Responder {
    // let posts=g
    let conn = pool.get().expect("Error when getting conn from pool");
    let res = get_list(&conn).await;
    let serialized = serde_json::to_string(&res).unwrap();
    // println!("{}", serialized);
    HttpResponse::build(http::StatusCode::OK)
        .content_type("application/json")
        .body(serialized)
}
