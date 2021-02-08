use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::models::user_category::{
    UserCategory,
    NewUserCategory
};

use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/user_category")]
pub async fn insert_user_category(
    authenticated_request: AuthenticatedRequest,
    mut new_user_category: web::Json<NewUserCategory>
) -> Result<HttpResponse, ElogError> {
    new_user_category.user_id = authenticated_request.user_id;
    UserCategory::insert(
        &authenticated_request.connection,
        new_user_category.0
    ).map(|_| {
        HttpResponse::Created().finish()
    })
}


#[get("/user_category")]
pub async fn get_all_user_categories(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    UserCategory::get_list(
        &authenticated_request.connection,
        authenticated_request.user_id
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}
