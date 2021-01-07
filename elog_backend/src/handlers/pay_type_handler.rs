use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::models::pay_type::{
    PayType,
    NewPayType,
    PayTypeList
};

use crate::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/pay_type")]
pub async fn insert_pay_type (
    authenticated_request: AuthenticatedRequest,
    pay_type: web::Json<NewPayType>
) -> Result<HttpResponse, ElogError> {
    PayType::insert(&authenticated_request.connection, pay_type.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/pay_type")]
pub async fn get_all_pay_types(authenticated_request: AuthenticatedRequest) -> HttpResponse {
    HttpResponse::Ok().json(PayTypeList::get_list(&authenticated_request.connection))
}
