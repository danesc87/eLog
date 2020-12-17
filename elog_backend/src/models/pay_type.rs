use serde::{Deserialize, Serialize};
use diesel::{
    MysqlConnection,
    insert_into,
    RunQueryDsl
};
use crate::error_handler::ElogError;

use super::schema::pay_type;
use super::schema::pay_type::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct PayType {
    pub id: i8,
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

    pub fn insert(connection: &MysqlConnection, new_pay_type: NewPayType) -> Result<usize, ElogError>{
        insert_into(pay_type)
            .values(&new_pay_type)
            .execute(connection)
            .map_err(|_| { ElogError::InsertFailure })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PayTypeList(pub Vec<PayType>);

impl PayTypeList {

    pub fn get_list(connection: &MysqlConnection) -> Self {
        let pay_types = pay_type
            .load::<PayType>(connection)
            .expect("Error during retrieving Pay Types");

        PayTypeList(pay_types)
    }
}
