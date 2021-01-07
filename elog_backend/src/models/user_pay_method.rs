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

use super::schema::user_pay_method;
use super::schema::user_pay_method::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPayMethod {
    pub id: i8,
    pub user_id: i16,
    pub pay_type_id: i8,
    pub bank_name: String,
    pub description: String,
    pub enabled: bool,
    pub register_date: NaiveDateTime
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[serde(default)]
#[table_name = "user_pay_method"]
pub struct NewUserPayMethod {
    pub user_id: i16,
    pub pay_type_id: i8,
    pub bank_name: String,
    pub description: String,
    pub enabled: bool
}

// Default implementation lets send JSON body without user_id and pay_type_id and fill them later
impl Default for NewUserPayMethod {
    fn default() -> Self {
        NewUserPayMethod {
            user_id: 0,
            pay_type_id: 0,
            bank_name: String::from(""),
            description: String::from(""),
            enabled: false
        }
    }
}

impl UserPayMethod {

    pub fn insert(connection: &SqlConnection, new_user_pay_method: NewUserPayMethod) -> Result<usize, ElogError> {
        insert_into(user_pay_method)
            .values(&new_user_pay_method)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_list(connection: &SqlConnection, logged_user_id: i16) -> Result<Vec<UserPayMethod>, ElogError> {
        user_pay_method
            .filter(user_id.eq(logged_user_id))
            .load::<UserPayMethod>(connection)
            .map_err(|_| { ElogError::ObjectNotFound(logged_user_id.to_string()) })
    }
}
