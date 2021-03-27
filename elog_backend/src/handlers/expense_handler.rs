use actix_web::{
    HttpResponse,
    web,
    get,
    post
};

use chrono::{Local, Datelike, NaiveDate, NaiveTime, NaiveDateTime};
use crate::models::expense::{
    Expense,
    NewExpense
};

use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/expense/{user_category_id}/{user_pay_method_id}")]
pub async fn insert_expense (
    authenticated_request: AuthenticatedRequest,
    dynamic_path: web::Path<(i16, i16,)>,
    mut new_expense: web::Json<NewExpense>
) -> Result<HttpResponse, ElogError> {
    new_expense.user_category_id = dynamic_path.clone().0;
    new_expense.user_pay_method_id = dynamic_path.into_inner().1;
    Expense::insert(
        &authenticated_request.connection,
        new_expense.0
    ).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/expense")]
pub async fn get_all_expenses(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    let current_date = Local::now().naive_utc();
    let beggining_date = NaiveDateTime::new(
        NaiveDate::from_ymd(current_date.year(), current_date.month(), 1),
        NaiveTime::from_hms_milli(0,0,0,0)
    );
    Expense::get_all_expenses(
        &authenticated_request.connection,
        authenticated_request.user_id,
        (beggining_date, current_date)
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}
