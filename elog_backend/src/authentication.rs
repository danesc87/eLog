use actix_web::{
    error::ErrorUnauthorized,
    web,
    dev,
    Error,
    FromRequest,
    HttpRequest
};
use futures::future::{err, ok, Ready};
use crate::utils::database_utils::{
    SqlPool,
    SqlPooledConnection,
    pool_handler
};
use crate::models::token::Claims;

pub struct AuthenticatedRequest{
    pub user_id: i16,
    pub string_token: String,
    pub connection: SqlPooledConnection
}

impl FromRequest for AuthenticatedRequest {
    type Error = Error;
    type Future = Ready<Result<AuthenticatedRequest, Error>>;
    type Config = ();

    fn from_request(http_request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = http_request.headers().get("Authorization");
        match auth {
            Some(_) => {
                let splitted_header_token: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = splitted_header_token[1].trim();
                let pool_from_app_data = http_request.app_data::<web::Data<SqlPool>>();
                let connection = pool_handler(pool_from_app_data);
                if Claims::is_valid_token(&connection.as_ref().clone().unwrap(), token) {
                    let decoded_token = Claims::decode_token(token);
                    match decoded_token {
                        Ok(_) => {
                            ok(
                                AuthenticatedRequest {
                                    user_id: decoded_token.unwrap().claims.id,
                                    string_token: token.to_owned(),
                                    connection: connection.unwrap()
                                }
                            )
                        },
                        Err(_) => err(ErrorUnauthorized("Invalid or expired token")),
                    }
                } else {
                    err(ErrorUnauthorized("Expired token"))
                }
            }
            None => err(ErrorUnauthorized("No token provided")),
        }
    }
}
