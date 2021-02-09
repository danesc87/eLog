use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    RunQueryDsl
};
use crate::utils::error_mapper::ElogError;

use super::schema::pay_type;
use super::schema::pay_type::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct PayType {
    pub id: i16,
    pub name: String,
    pub description: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "pay_type"]
pub struct NewPayType {
    pub name: String,
    pub description: String
}

impl PayType {

    pub fn insert(
        connection: &SqlConnection,
        new_pay_type: NewPayType
    ) -> Result<usize, ElogError> {
        insert_into(pay_type)
            .values(&new_pay_type)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_list(connection: &SqlConnection) -> Result<Vec<PayType>, ElogError> {
        pay_type
            .load::<PayType>(connection)
            .map_err(|error| { ElogError::ErrorRetrievingData(error.to_string()) })
    }
}
