use serde::Serialize;
use actix_web::{HttpResponse, error::ResponseError};

#[derive(Fail, Debug, Serialize)]
pub enum ElogError {
    #[fail(display="{} Bad Request", _0)]
    InsertFailure(String),
    #[fail(display="{} Not found", _0)]
    ObjectNotFound(String),
    #[fail(display="{} Error Retrieving from DB", _0)]
    ErrorRetrievingData(String),
    #[fail(display="{} TokenCreationError", _0)]
    TokenCreationError(String),
}

impl ResponseError for ElogError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ElogError::InsertFailure(ref message) => HttpResponse::BadRequest()
                .json(message),
            ElogError::ObjectNotFound(ref message) => HttpResponse::NotFound()
                .json(message),
            ElogError::ErrorRetrievingData(ref message) => HttpResponse::BadRequest()
                .json(message),
            ElogError::TokenCreationError(ref message) => HttpResponse::InternalServerError()
                .json(message),
        }
    }
}
