use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::models::user_pay_type::{
    UserPayType,
    NewUserPayType
};

use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/user_pay_type")]
pub async fn insert_pay_type (
    authenticated_request: AuthenticatedRequest,
    mut new_user_pay_type: web::Json<NewUserPayType>
) -> Result<HttpResponse, ElogError> {
    new_user_pay_type.user_id = authenticated_request.user_id;
    UserPayType::insert(&authenticated_request.connection, new_user_pay_type.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/user_pay_type")]
pub async fn get_all_pay_types(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    UserPayType::get_list(&authenticated_request.connection).map(|list| {
        HttpResponse::Ok().json(list)
    })
}
