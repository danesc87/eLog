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

mod config;
mod authentication;
mod handlers;
mod models;
mod utils;
use config::route_config;
use config::get_cors;
use utils::database_utils::connect_database;
use utils::env_variable_utils::get_variable;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init Dotenv and EnvLogger
    dotenv::dotenv().ok();
    env_logger::init();

    let server_url = format!("{}:{}", get_variable("SERVER_IP"), get_variable("SERVER_PORT"));
    println!("\nServer Running on: {}\n", server_url);

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(get_cors())
            .data(connect_database())
            .configure(route_config)
    })
    .bind(server_url)?
    .run()
    .await
}
