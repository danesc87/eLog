use serde::{Deserialize, Serialize};
use diesel::{
    MysqlConnection,
    insert_into,
    RunQueryDsl
};

use chrono::NaiveDateTime;

use super::schema::expense;
use super::schema::expense::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub user_pay_method_id: i8,
    pub ammount: f64,
    pub description: String,
    pub register_date: NaiveDateTime
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "expense"]
pub struct NewExpense {
    pub user_pay_method_id: i8,
    pub ammount: f64,
    pub description: String
}

impl Expense {

    pub fn insert(connection: &MysqlConnection, new_expense: NewExpense) {
        let _insert = insert_into(expense)
            .values(&new_expense)
            .execute(connection)
            .map_err(|error| {
                println!("Error during insert expense: {:?}, {:?}", new_expense, error);
                error
            });
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExpenseList(pub Vec<Expense>);

impl ExpenseList {

    pub fn get_list(connection: &MysqlConnection) -> Self {
        let expenses = expense
            .load::<Expense>(connection)
            .expect("Error during retrieving expenses");

        ExpenseList(expenses)
    }
}
