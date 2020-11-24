use std::io::Result;
use actix_web::{
    HttpServer,
    App,
    middleware
};

#[macro_use]
extern crate diesel;

mod config;
mod handlers;
mod models;
use config::route_config;
use config::connect;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
        .data(connect())
        .wrap(middleware::Logger::default())
        .configure(route_config)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
