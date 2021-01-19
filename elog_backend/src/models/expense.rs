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
    pub user_pay_method_id: i16,
    pub ammount: f64,
    pub description: String,
    pub register_date: NaiveDateTime
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[serde(default)]
#[table_name = "expense"]
pub struct NewExpense {
    pub user_pay_method_id: i16,
    pub ammount: f64,
    pub description: String
}

// Default implementation lets send JSON body without user_pay_method_id
impl Default for NewExpense {
    fn default() -> Self {
        NewExpense {
            user_pay_method_id: 0,
            ammount: 0.0,
            description: String::from("")
        }
    }
}

impl Expense {

    pub fn insert(connection: &SqlConnection, new_expense: NewExpense) -> Result<usize, ElogError> {
        insert_into(expense)
            .values(&new_expense)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_list(connection: &SqlConnection, logged_user_id: i16) -> Result<Vec<Expense>, ElogError> {
        use super::schema::user_pay_method::dsl::*;
        let current_user_pay_method_id = user_pay_method
            .filter(user_id.eq(logged_user_id))
            .select(id)
            .first::<i16>(connection)
            .map_err(|_| { ElogError::ObjectNotFound(logged_user_id.to_string()) });

        match current_user_pay_method_id {
            Ok(method_id) => {
                expense
                    .filter(user_pay_method_id.eq(method_id))
                    .load::<Expense>(connection)
                    .map_err(|_| { ElogError::ObjectNotFound(logged_user_id.to_string()) })
            },
            Err(error) => Err(error),
        }
    }
}
