use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct UserPayMethod {
    pub id: u32,
    pub user_id: u32,
    pub location: String,
    pub bank_name: String,
    pub description: String,
    pub status: bool,
    pub register_date: Timestamp
}
