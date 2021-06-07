use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    get,
    post,
    put
};
use crate::utils::database_utils::{
    SqlPool,
    pool_handler
};
use crate::models::app_user::{
    AppUser,
    NewAppUser,
    LoginAppUser
};
use crate::models::session_properties::SessionProperties;

use crate::utils::{
    error_mapper::ElogError,
    http_request_utils::get_token_from_auth_header
};
use crate::authentication::AuthenticatedRequest;

#[post("/register")]
pub async fn register(
    pool: web::Data<SqlPool>,
    app_user: web::Json<NewAppUser>
) -> Result<HttpResponse, ElogError> {
    let connection = pool_handler(Some(&pool));
    AppUser::register(&connection.unwrap(), app_user.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[post("/login")]
pub async fn login(
    pool: web::Data<SqlPool>,
    login_app_user: web::Json<LoginAppUser>,
) -> Result<HttpResponse, ElogError> {
    let connection = pool_handler(Some(&pool));
    AppUser::login(&connection.unwrap(), login_app_user.0).map(|token| {
        HttpResponse::Ok().json(token)
    })
}

#[get("/logout")]
pub async fn logout(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    AppUser::logout(
        &authenticated_request.connection,
        authenticated_request.string_token.as_str()
    ).map(|_| {
        HttpResponse::Ok().finish()
    })
}

#[get("/session_properties")]
pub async fn session_properties(
    pool: web::Data<SqlPool>,
    http_request: HttpRequest
) -> HttpResponse {
    let auth = http_request.headers().get("Authorization");
    match auth {
        Some(_) => {
            let connection = pool_handler(Some(&pool));
            HttpResponse::Ok().json(
                SessionProperties::get_session_properties(
                    &connection.unwrap(),
                    get_token_from_auth_header(auth)
                )
            )
        },
        None => HttpResponse::Ok().json(SessionProperties::no_token_session_properties())
    }
}

#[put("/settings")]
pub async fn update_app_user(
    authenticated_request: AuthenticatedRequest,
    app_user: web::Json<NewAppUser>
) -> Result<HttpResponse, ElogError> {
    AppUser::update(
        &authenticated_request.connection,
        authenticated_request.user_id,
        app_user.into_inner()
    ).map(|_| { HttpResponse::Ok().finish() })
}
