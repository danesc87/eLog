use serde::Serialize;
use actix_web::{HttpResponse, http::StatusCode, error::ResponseError};

#[derive(Fail, Debug, Serialize)]
pub enum ElogError {
    #[fail(display="Bad Request")]
    InsertFailure,
    #[fail(display="{} Not found", _0)]
    ObjectNotFound(String),
    #[fail(display="TokenCreationError")]
    TokenCreationError,
}

impl ResponseError for ElogError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ElogError::InsertFailure => HttpResponse::new(
                StatusCode::BAD_REQUEST
            ),
            ElogError::ObjectNotFound(ref message) => HttpResponse::NotFound()
                .json(message),
            ElogError::TokenCreationError => HttpResponse::InternalServerError()
                .json("Token cannot be created"),
        }
    }
}
