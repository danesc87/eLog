use serde::Serialize;
use actix_web::{HttpResponse, http::StatusCode, error::ResponseError};

#[derive(Fail, Debug, Serialize)]
pub enum ElogError {
    #[fail(display="Bad Request")]
    InsertFailure,
    #[fail(display="Operation Not Permitted")]
    OperationNotPermitted,
}

impl ResponseError for ElogError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ElogError::InsertFailure => HttpResponse::new(
                StatusCode::BAD_REQUEST
            ),
            ElogError::OperationNotPermitted => HttpResponse::new(
                StatusCode::FORBIDDEN
            ),
        }
    }
}
