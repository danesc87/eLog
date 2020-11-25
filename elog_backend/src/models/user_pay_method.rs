use serde::{Deserialize, Serialize};
use diesel::{
    MysqlConnection,
    insert_into,
    RunQueryDsl
};

use chrono::NaiveDateTime;

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

    pub fn insert(connection: &MysqlConnection, new_user_pay_method: NewUserPayMethod) {
        let _insert = insert_into(user_pay_method)
            .values(&new_user_pay_method)
            .execute(connection)
            .map_err(|error| {
                println!("Error during insert user pay method: {:?}, {:?}", new_user_pay_method, error);
                error
            });
    }
}


#[derive(Serialize, Deserialize)]
pub struct UserPayMethodList(pub Vec<UserPayMethod>);

impl UserPayMethodList {

    pub fn get_list(connection: &MysqlConnection) -> Self {
        let user_pay_methods = user_pay_method
            .load::<UserPayMethod>(connection)
            .expect("Error during retrieving User Pay Methods");

        UserPayMethodList(user_pay_methods)
    }
}
