use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct AppUser {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role_id: u8,
    pub status: bool,
    pub register_date: Timestamp
}
