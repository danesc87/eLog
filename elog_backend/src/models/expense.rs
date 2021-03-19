use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use chrono::NaiveDateTime;

use crate::utils::error_mapper::ElogError;
use super::schema::expense;
use super::schema::expense::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub user_category_id: i16,
    pub user_pay_method_id: i16,
    pub amount: f64,
    pub description: String,
    pub register_date: NaiveDateTime
}

#[derive(Debug, Insertable, Deserialize)]
#[serde(default)]
#[table_name = "expense"]
pub struct NewExpense {
    pub user_category_id: i16,
    pub user_pay_method_id: i16,
    pub amount: f64,
    pub description: String
}

// Default implementation lets send JSON body without user_pay_method_id
impl Default for NewExpense {
    fn default() -> Self {
        NewExpense {
            user_category_id: 0,
            user_pay_method_id: 0,
            amount: 0.0,
            description: String::from("")
        }
    }
}

#[derive(Queryable, Serialize)]
pub struct ExpenseWithCategoriesAndPayMethods {
    pub id: i32,
    pub user_category: String,
    pub user_pay_method: String,
    pub amount: f64,
    pub description: String,
    pub register_date: NaiveDateTime
}

impl Expense {

    pub fn insert(
        connection: &SqlConnection,
        new_expense: NewExpense
    ) -> Result<usize, ElogError> {
        insert_into(expense)
            .values(&new_expense)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_all_expenses(
        connection: &SqlConnection,
        logged_user_id: i16,
        naive_date_times: (NaiveDateTime, NaiveDateTime)
    ) -> Result<Vec<ExpenseWithCategoriesAndPayMethods>, ElogError> {
        use super::schema::{user_category, user_pay_method};
        
        expense
            .inner_join(user_category::table)
            .inner_join(user_pay_method::table)
            .filter(user_category::user_id.eq(user_pay_method::user_id))
            .filter(user_category::user_id.eq(logged_user_id))
            .filter(register_at.between(naive_date_times.0, naive_date_times.1))
            .select((
                expense::id,
                user_category::category,
                user_pay_method::bank_name,
                expense::amount,
                expense::description,
                expense::register_at
            ))
            .load::<ExpenseWithCategoriesAndPayMethods>(connection)
            .map_err(|_| { ElogError::ObjectNotFound(logged_user_id.to_string()) })
    }
}
