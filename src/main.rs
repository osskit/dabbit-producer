use actix_web::{web, App, HttpServer};
use framework::environment::get_environment;
use server::config;

mod framework;
mod server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = get_environment().expect("failed to load env");
    let environment_data = web::Data::new(environment);

    HttpServer::new(move || {
        App::new()
            .app_data(environment_data.clone())
            .configure(config)
    })
    .bind(("0.0.0.0", environment.port))?
    .run()
    .await
}
