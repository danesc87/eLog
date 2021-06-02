use actix_web::web;
use actix_cors::Cors;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

// Endpoints registration config
pub fn routes(config: &mut web::ServiceConfig) {
    // Only imports of Endpoints
    use crate::handlers::{
        app_user_handler::{
            register,
            login,
            logout,
            session_properties,
            update_app_user
        },
        user_pay_type_handler::{
            insert_user_pay_type,
            get_all_user_pay_types,
            update_user_pay_type,
            delete_user_pay_type
        },
        user_category_handler::{
            insert_user_category,
            get_all_user_categories,
            update_user_category,
            delete_user_category
        },
        expense_handler::{
            insert_expense,
            get_all_expenses,
            delete_expense
        },
        report_handler::get_expenses_for_report,
    };

    config
        .service(register)
        .service(login)
        .service(logout)
        .service(session_properties)
        .service(update_app_user)
        .service(insert_user_pay_type)
        .service(get_all_user_pay_types)
        .service(update_user_pay_type)
        .service(delete_user_pay_type)
        .service(insert_user_category)
        .service(get_all_user_categories)
        .service(update_user_category)
        .service(delete_user_category)
        .service(insert_expense)
        .service(get_all_expenses)
        .service(delete_expense)
        .service(get_expenses_for_report);
}
