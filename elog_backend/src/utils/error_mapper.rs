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
    #[fail(display="{} BadQueryParameters", _0)]
    BadQueryParameters(String),
}

#[derive(Serialize)]
struct ErrorMessage {
    cause: String,
    message: String
}

impl ErrorMessage {
    fn new(cause: &str, message: &String) -> Self {
        ErrorMessage {
            cause: cause.to_string(),
            message: message.as_str().to_string()
        }
    }
}

impl ResponseError for ElogError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ElogError::InsertFailure(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("InsertFailure", message)),
            ElogError::ObjectNotFound(ref message) => HttpResponse::NotFound()
                .json(ErrorMessage::new("ObjectNotFound", message)),
            ElogError::ErrorRetrievingData(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("ErrorRetrievingData", message)),
            ElogError::TokenCreationError(ref message) => HttpResponse::InternalServerError()
                .json(ErrorMessage::new("TokenCreationError", message)),
            ElogError::BadQueryParameters(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("BadQueryParameters", message))
        }
    }
}
