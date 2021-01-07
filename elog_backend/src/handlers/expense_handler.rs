use actix_web::{
    HttpResponse,
    web,
    get,
    post
};
use crate::models::expense::{
    Expense,
    NewExpense,
    ExpenseList
};

use crate::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/expense")]
pub async fn insert_expense (
    new_expense: web::Json<NewExpense>,
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    Expense::insert(&authenticated_request.connection, new_expense.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/expense")]
pub async fn get_all_expenses(authenticated_request: AuthenticatedRequest) -> HttpResponse {
    HttpResponse::Ok().json(ExpenseList::get_list(&authenticated_request.connection))
}
