use actix_web::{
    HttpResponse,
    get,
    web,
};
use serde::Deserialize;
use chrono::{
    Duration,
    Local,
    NaiveDateTime
};

use crate::models::expense_report::ExpenseForReport;
use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[derive(Deserialize, Debug)]
pub struct DateQueryParameters {
    since_when: Option<i64>,
    until_when: Option<i64>
}


#[get("/report/expense")]
pub async fn get_expenses_for_report(
    authenticated_request: AuthenticatedRequest,
    date_query_parameters: web::Query<DateQueryParameters>
) -> Result<HttpResponse, ElogError> {
    ExpenseForReport::get_expenses_for_report(
        &authenticated_request.connection,
        authenticated_request.user_id,
        get_report_naive_date_times(date_query_parameters.0)
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}

fn get_report_naive_date_times(
    date_query_parameters: DateQueryParameters
) -> (NaiveDateTime, NaiveDateTime) {
    use std::i64;
    let since_millis = Duration::milliseconds(date_query_parameters
        .since_when
        .unwrap_or(0))
        .to_std().unwrap();
    let until_millis = Duration::milliseconds(date_query_parameters
        .until_when
        .unwrap_or(Local::now().timestamp_millis()))
        .to_std().unwrap();

    (
        NaiveDateTime::from_timestamp(
            since_millis.as_secs() as i64,
            since_millis.subsec_nanos()
        ),
        NaiveDateTime::from_timestamp(
            until_millis.as_secs() as i64,
            until_millis.subsec_nanos()
        )
    )
}
