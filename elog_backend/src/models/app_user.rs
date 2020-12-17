use serde::{Deserialize, Serialize};
use diesel::{
    MysqlConnection,
    insert_into,
    RunQueryDsl
};

use chrono::NaiveDateTime;
use crate::error_handler::ElogError;

use super::schema::app_user;
use super::schema::app_user::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i16,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub register_at: NaiveDateTime
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "app_user"]
pub struct NewAppUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String
}

impl AppUser {

    pub fn register(connection: & MysqlConnection, new_user: NewAppUser) -> Result<usize, ElogError> {
        insert_into(app_user)
            .values(&new_user)
            .execute(connection)
            .map_err(|_| { ElogError::InsertFailure })
    }
}
