use serde::Serialize;
use std::collections::HashMap;
use crate::utils::database_utils::SqlConnection;
use diesel::{
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use chrono::NaiveDateTime;

use crate::utils::error_mapper::ElogError;
use super::expense::ExpenseWithCategoriesAndPayMethods;
use super::schema::expense;
use super::schema::expense::dsl::*;

#[derive(Serialize)]
pub struct ExpenseByGroup {
    pub amount: f64,
    pub category: String
}

#[derive(Serialize)]
pub struct ExpenseForReport {
    pub expenses_by_group: Vec<ExpenseByGroup>,
    pub since_when: NaiveDateTime,
    pub until_when: NaiveDateTime,
}


impl ExpenseForReport {
    pub fn get_expenses_for_report(
        connection: &SqlConnection,
        logged_user_id: i16,
        naive_date_times: (NaiveDateTime, NaiveDateTime)
    ) -> Result<ExpenseForReport, ElogError> {
        use super::schema::{user_category, user_pay_method};
        let all_expenses = expense
            .inner_join(user_category::table)
            .inner_join(user_pay_method::table)
            .filter(user_category::user_id.eq(user_pay_method::user_id))
            .filter(user_category::user_id.eq(logged_user_id))
            .filter(register_at.ge(naive_date_times.0))
            .filter(register_at.le(naive_date_times.1))
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
            Ok(
                ExpenseForReport {
                    expenses_by_group: Self::get_all_report_expenses(expense_map),
                    since_when: naive_date_times.0,
                    until_when: naive_date_times.1
                }
            )
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

    fn get_all_report_expenses(expense_map: HashMap<String, f64>) -> Vec<ExpenseByGroup> {
        let mut expense_for_report: Vec<ExpenseByGroup> = Vec::new();
        for (category, grouped_amount) in expense_map {
            expense_for_report.push(
                ExpenseByGroup { amount: grouped_amount, category }
            );
        }
        expense_for_report
    }
}