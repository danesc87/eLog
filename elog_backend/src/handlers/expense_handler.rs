use actix_web::{
    HttpResponse,
    web,
    get,
    post
};

use crate::config::{
    MySqlPool,
    mysql_pool_handler
};

use crate::models::expense::{
    Expense,
    NewExpense,
    ExpenseList
};

use crate::error_mapper::ElogError;
use crate::authentication::AuthorizationService;

#[post("/expense")]
pub async fn insert_expense (
    pool: web::Data<MySqlPool>,
    new_expense: web::Json<NewExpense>
) -> Result<HttpResponse, ElogError> {
    let connection = mysql_pool_handler(pool);
    Expense::insert(&connection.unwrap(), new_expense.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/expense")]
pub async fn get_all_expenses(
    pool: web::Data<MySqlPool>,
    _: AuthorizationService
) -> HttpResponse {
    let connection = mysql_pool_handler(pool);
    HttpResponse::Ok().json(ExpenseList::get_list(&connection.unwrap()))
}
