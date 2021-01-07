use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::config::{
    MySqlPool,
    mysql_pool_handler
};
use crate::models::app_user::{
    AppUser,
    NewAppUser,
    LoginAppUser
};

use crate::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/register")]
pub async fn register(
    pool: web::Data<MySqlPool>,
    app_user: web::Json<NewAppUser>
) -> Result<HttpResponse, ElogError> {
    let connection = mysql_pool_handler(Some(&pool));
    AppUser::register(&connection.unwrap(), app_user.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[post("/login")]
pub async fn login(
    pool: web::Data<MySqlPool>,
    login_app_user: web::Json<LoginAppUser>,
) -> Result<HttpResponse, ElogError> {
    let connection = mysql_pool_handler(Some(&pool));
    AppUser::login(&connection.unwrap(), login_app_user.0).map(|token| {
        HttpResponse::Ok().json(token)
    })
}

#[get("/logout")]
pub async fn logout(authenticated_request: AuthenticatedRequest) -> Result<HttpResponse, ElogError> {
    AppUser::logout(
        &authenticated_request.connection,
        authenticated_request.string_token.as_str()
    ).map(|_| {
        HttpResponse::Ok().finish()
    })
}
