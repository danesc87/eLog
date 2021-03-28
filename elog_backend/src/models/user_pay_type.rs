use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    RunQueryDsl
};
use crate::utils::error_mapper::ElogError;

use super::schema::user_pay_type;
use super::schema::user_pay_type::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPayType {
    pub id: i16,
    pub user_id: i16,
    pub name: String,
    pub bank_name: String,
    pub description: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[serde(default)]
#[table_name = "user_pay_type"]
pub struct NewUserPayType {
    pub user_id: i16,
    pub name: String,
    pub bank_name: String,
    pub description: String
}

// Default implementation lets send JSON body without user_id
impl Default for NewUserPayType {
    fn default() -> Self {
        NewUserPayType {
            user_id: 0,
            name: String::from(""),
            bank_name: String::from(""),
            description: String::from("")
        }
    }
}


impl UserPayType {

    pub fn insert(
        connection: &SqlConnection,
        new_user_pay_type: NewUserPayType
    ) -> Result<usize, ElogError> {
        insert_into(user_pay_type)
            .values(&new_user_pay_type)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_list(connection: &SqlConnection) -> Result<Vec<UserPayType>, ElogError> {
        user_pay_type
            .load::<UserPayType>(connection)
            .map_err(|error| { ElogError::ErrorRetrievingData(error.to_string()) })
    }
}
