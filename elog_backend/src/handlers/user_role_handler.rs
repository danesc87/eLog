use actix_web::{
    HttpResponse,
    web,
    get
};
use crate::models::user_role::{
    UserRole,
    UserRoleList
};

use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[get("/user_roles")]
pub async fn get_user_roles(authenticated_request: AuthenticatedRequest) -> Result<HttpResponse, ElogError> {
    Ok(HttpResponse::Ok().json(UserRoleList::list(&authenticated_request.connection)))
}

#[get("/user_roles/{id}")]
pub async fn get_user_role_with_id(
    authenticated_request: AuthenticatedRequest,
    path: web::Path<(i16,)>
) -> Result<HttpResponse, ElogError> {
    Ok(HttpResponse::Ok().json(UserRole::get_with_id(&authenticated_request.connection, path.into_inner().0)))
}
