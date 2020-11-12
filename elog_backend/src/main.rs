use actix_web::{
    HttpServer,
    App,
    middleware
};

mod config;
use config::route_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .configure(route_config)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}