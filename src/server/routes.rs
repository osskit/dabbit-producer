use actix_web::{get, post, web, HttpResponse, Responder};

use crate::framework::environment::Environment;

#[get("/")]
pub async fn hello(environment: web::Data<Environment>) -> impl Responder {
    println!("port is now: {}", environment.port);

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