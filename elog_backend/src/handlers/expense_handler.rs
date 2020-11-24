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

#[post("/expense")]
pub async fn insert_expense (
    pool: web::Data<MySqlPool>,
    new_expense: web::Json<NewExpense>
) -> HttpResponse {
    let connection = mysql_pool_handler(pool);
    HttpResponse::Created().json(Expense::insert(&connection.unwrap(), new_expense.0))
}

#[get("/expense")]
pub async fn get_all_expenses(pool: web::Data<MySqlPool>) -> HttpResponse {
    let connection = mysql_pool_handler(pool);
    HttpResponse::Ok().json(ExpenseList::get_list(&connection.unwrap()))
}
