use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use chrono::NaiveDateTime;
use chrono::Duration;
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

#[derive(Queryable, Serialize)]
pub struct ExpenseForReport {
    pub amount: f64,
    pub category: String
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

    pub fn get_list(
        connection: &SqlConnection,
        logged_user_id: i16
    ) -> Result<Vec<ExpenseWithCategoriesAndPayMethods>, ElogError> {
        use super::schema::{user_category, user_pay_method};
        expense
            .inner_join(user_category::table)
            .inner_join(user_pay_method::table)
            .filter(user_category::user_id.eq(user_pay_method::user_id))
            .filter(user_category::user_id.eq(logged_user_id))
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

    pub fn get_expenses_for_report(
        connection: &SqlConnection,
        logged_user_id: i16
    ) -> Result<Vec<ExpenseForReport>, ElogError> {
        use super::schema::{user_category, user_pay_method};
        let all_expenses= expense
            .inner_join(user_category::table)
            .inner_join(user_pay_method::table)
            .filter(user_category::user_id.eq(user_pay_method::user_id))
            .filter(user_category::user_id.eq(logged_user_id))
            .select((
                expense::id,
                user_category::category,
                user_pay_method::bank_name,
                expense::amount,
                expense::description,
                expense::register_at
            ))
            .load::<ExpenseWithCategoriesAndPayMethods>(connection)
            .map_err(|_| { ElogError::ObjectNotFound(logged_user_id.to_string()) });
        if all_expenses.is_ok() {
            let expense_map = Self::insert_or_update_category_amount_on_map(all_expenses.unwrap());
            Ok(Self::get_all_report_expenses(expense_map))
        } else {
            Err(all_expenses.err().unwrap())
        }
    }

    fn insert_or_update_category_amount_on_map(
        all_expenses: Vec<ExpenseWithCategoriesAndPayMethods>
    ) -> HashMap<String, f64> {
        let mut expense_map: HashMap<String, f64> = HashMap::new();
        for each_expense in &all_expenses {
            let category: String = each_expense.user_category.clone();
            match expense_map.get(&category) {
                Some(_) => expense_map.insert(category.clone(), each_expense.amount + expense_map[&category]),
                None => expense_map.insert(category, each_expense.amount)
            };
        }
        expense_map
    }

    fn get_all_report_expenses(expense_map: HashMap<String, f64>) -> Vec<ExpenseForReport> {
        let mut expense_for_report: Vec<ExpenseForReport> = Vec::new();
        for (category, a) in expense_map {
            expense_for_report.push(
                ExpenseForReport { amount: a, category }
            );
        }
        expense_for_report
    }
}
