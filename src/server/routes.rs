use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ready")]
pub async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/produce")]
pub async fn produce() -> impl Responder {
    HttpResponse::Ok()
}