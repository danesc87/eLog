use actix_web::{
    HttpResponse,
    get,
    web,
};

use super::parameters::DateQueryParameters;
use crate::models::expense_report::ExpenseForReport;
use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;




#[get("/report/expense")]
pub async fn get_expenses_for_report(
    authenticated_request: AuthenticatedRequest,
    date_query_parameters: web::Query<DateQueryParameters>
) -> Result<HttpResponse, ElogError> {
    let naive_date_times = date_query_parameters.0.get_naive_date_time_from_millis();
    if naive_date_times.is_err() {
        return Err(naive_date_times.err().unwrap());
    }
    ExpenseForReport::get_expenses_for_report(
        &authenticated_request.connection,
        authenticated_request.user_id,
        naive_date_times.unwrap()
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}
