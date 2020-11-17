use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct Expense {
    pub id: u32,
    pub user_pay_method_id: u32,
    pub expense_value: f32,
    pub register_date: Timestamp
}
