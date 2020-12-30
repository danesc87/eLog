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
mod authentication;
mod handlers;
mod models;
mod error_mapper;
use config::route_config;
use config::connect;
use config::get_cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(get_cors())
            .data(connect())
            .configure(route_config)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
