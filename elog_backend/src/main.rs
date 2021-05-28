use actix_web::{
    HttpServer,
    App,
    middleware::Logger
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

mod route_config;
mod server_config;
mod authentication;
mod handlers;
mod models;
mod utils;
use route_config::routes;
use route_config::get_cors;
use utils::database_utils::connect_database;
use server_config::ElogConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let elog_config = ElogConfig::new();
    std::env::set_var("RUST_LOG", elog_config.log_type);

    // Init EnvLogger
    env_logger::init();
    let server_url = format!("{}:{}", elog_config.ip_address, elog_config.server_port);
    println!("\nServer Running on: {}\n", server_url);

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(get_cors())
            .data(connect_database())
            .configure(routes)
    })
    .bind(server_url)?
    .run()
    .await
}
