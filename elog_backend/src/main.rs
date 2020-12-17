use actix_web::{
    HttpServer,
    App,
    middleware::Logger
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

mod config;
mod handlers;
mod models;
mod error_handler;
use config::route_config;
use config::connect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .data(connect())
            .wrap(Logger::default())
            .configure(route_config)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
