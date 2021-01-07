use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::models::user_pay_method::{
    UserPayMethod,
    NewUserPayMethod,
    UserPayMethodList
};

use crate::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/user_pay_method/{user_id}/{pay_type_id}")]
pub async fn insert_user_pay_method (
    path: web::Path<(i16, i8)>,
    mut new_user_pay_method: web::Json<NewUserPayMethod>,
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError>  {
    let unwraped_path = path.into_inner();
    new_user_pay_method.user_id = unwraped_path.0;
    new_user_pay_method.pay_type_id = unwraped_path.1;
    UserPayMethod::insert(&authenticated_request.connection, new_user_pay_method.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/user_pay_method")]
pub async fn get_all_user_pay_methods(authenticated_request: AuthenticatedRequest) -> HttpResponse {
    HttpResponse::Ok().json(UserPayMethodList::get_list(&authenticated_request.connection))
}
