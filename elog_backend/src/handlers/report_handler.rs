use actix_web::{
    HttpResponse,
    get,
};

use crate::models::expense::Expense;
use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[get("/report/expense")]
pub async fn get_expenses_for_report(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    Expense::get_expenses_for_report(
        &authenticated_request.connection,
        authenticated_request.user_id
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}
