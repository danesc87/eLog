use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::models::user_pay_method::{
    UserPayMethod,
    NewUserPayMethod
};

use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/user_pay_method/{pay_type_id}")]
pub async fn insert_user_pay_method(
    authenticated_request: AuthenticatedRequest,
    dynamic_path: web::Path<(i16,)>,
    mut new_user_pay_method: web::Json<NewUserPayMethod>
) -> Result<HttpResponse, ElogError>  {
    new_user_pay_method.user_id = authenticated_request.user_id;
    new_user_pay_method.pay_type_id = dynamic_path.into_inner().0;
    UserPayMethod::insert(
        &authenticated_request.connection,
        new_user_pay_method.0
     ).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/user_pay_method")]
pub async fn get_all_user_pay_methods(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    UserPayMethod::get_list(
        &authenticated_request.connection,
        authenticated_request.user_id
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}
