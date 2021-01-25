use actix_web::{get, http, post, web, HttpRequest, HttpResponse, Responder};
use std::{fs, time};
#[get("/")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    // Ok(HttpResponse::Ok().body("hello"))
    HttpResponse::Ok().body("hello")
}

#[get("/artical/{filepath}")]
pub async fn blog(_req: HttpRequest, web::Path(filepath): web::Path<String>) -> impl Responder {
    println!("{}", filepath);
    let url = format! {"blog/{}",filepath};
    let artical =
        fs::read_to_string(url).expect(&format!("an Err occurs while opening {}", filepath));
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html")
        .body(artical)
}

#[post("/artical")]
async fn post_artical(_req: HttpRequest, filename: String) -> impl Responder {
    "Ok"
}
