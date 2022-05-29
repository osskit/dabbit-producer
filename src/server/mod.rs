use actix_web::web;

pub mod routes;

use routes::{hello, produce, ready};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ready);
    cfg.service(hello);
    cfg.service(produce);
}
