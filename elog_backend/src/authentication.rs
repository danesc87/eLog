use actix_web::{
    error::ErrorUnauthorized,
    dev,
    Error,
    FromRequest,
    HttpRequest
};
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    Validation
};
use futures::future::{err, ok, Ready};

use crate::models::token::Claims;
use crate::config::get_secret_key;

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
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(get_secret_key().as_bytes()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_) => ok(AuthorizationService),
                    Err(_) => err(ErrorUnauthorized("Invalid token")),
                }
            }
            None => err(ErrorUnauthorized("Token expired or inexistent")),
        }
    }
}
