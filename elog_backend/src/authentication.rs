use actix_web::{
    error::ErrorUnauthorized,
    dev,
    Error,
    FromRequest,
    HttpRequest
};
use futures::future::{err, ok, Ready};

use crate::models::token::Claims;

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(http_request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = http_request.headers().get("Authorization");
        match auth {
            Some(_) => {
                let splitted_header_token: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = splitted_header_token[1].trim();
                // todo!(Search a way to check if current token is on blacklisted tokens)
                match Claims::decode_token(token) {
                    Ok(_) => ok(AuthorizationService),
                    Err(_) => err(ErrorUnauthorized("Invalid or expired token")),
                }
            }
            None => err(ErrorUnauthorized("No token provided")),
        }
    }
}
