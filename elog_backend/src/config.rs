use actix_web::web;
use actix_cors::Cors;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

// Endpoints registration config
pub fn route_config(config: &mut web::ServiceConfig) {
    // Only imports of Endpoints
    use crate::handlers::{
        app_user_handler::{
            register,
            login,
            logout,
            session_properties
        },
        pay_type_handler::{
            insert_pay_type,
            get_all_pay_types
        },
        user_pay_method_handler::{
            insert_user_pay_method,
            get_all_user_pay_methods
        },
        expense_handler::{
            insert_expense,
            get_all_expenses
        }
    };

    config
        .service(register)
        .service(login)
        .service(logout)
        .service(session_properties)
        .service(insert_pay_type)
        .service(get_all_pay_types)
        .service(insert_user_pay_method)
        .service(get_all_user_pay_methods)
        .service(insert_expense)
        .service(get_all_expenses);
}
