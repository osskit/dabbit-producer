use actix_web::web;

pub mod routes;

use routes::{hello, ready};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ready);
    cfg.service(hello);
}
