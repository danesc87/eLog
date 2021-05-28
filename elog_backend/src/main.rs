use actix_web::{
    HttpServer,
    App,
    middleware::Logger
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

mod route_config;
mod server_config;
mod authentication;
mod handlers;
mod models;
mod utils;
use route_config::routes;
use route_config::get_cors;
use utils::database_utils::connect_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Put log type as env variable, since env_logger use it
    std::env::set_var("RUST_LOG", server_config::ELOG_CONFIG.clone().log_type);
    // Init EnvLogger
    env_logger::init();

    let server_url = format!(
        "{}:{}",
        server_config::ELOG_CONFIG.ip_address,
        server_config::ELOG_CONFIG.server_port
    );
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
