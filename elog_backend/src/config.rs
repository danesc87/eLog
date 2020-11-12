use actix_web::web;
use elog_backend::elog::{
    hello_world,
    hello_user_name,
    test_sum
};

// To make and endpoint accessible should be registered in this method
pub fn route_config(config: &mut web::ServiceConfig) {
    config.service(hello_world)
        .service(hello_user_name)
        .service(test_sum);
}
