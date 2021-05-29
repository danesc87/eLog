use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    QueryDsl,
    insert_into,
    RunQueryDsl,
    ExpressionMethods
};

use chrono::NaiveDateTime;
use super::token::Claims;
use crate::utils::error_mapper::ElogError;

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

// Struct to allow send app user data for future login
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "app_user"]
pub struct LoginAppUser {
    pub username: String,
    pub password: String
}

// Struct to show logged app user as a response with the token type and jwt token
#[derive(Debug, Serialize)]
pub struct AppUserToken {
    pub token_type: String,
    pub access_token: String
}

impl AppUser {

    pub fn register(
        connection: &SqlConnection,
        mut new_user: NewAppUser
    ) -> Result<usize, ElogError> {
        use data_encoding::BASE64;
        new_user.password = BASE64.encode(new_user.password.as_bytes());
        insert_into(app_user)
            .values(&new_user)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn login(
        connection: &SqlConnection,
        login_app_user: LoginAppUser
    ) -> Result<AppUserToken, ElogError> {
        use data_encoding::BASE64;
        let logged_app_user = app_user
            .filter(username.eq(login_app_user.username.clone()))
            .filter(password.eq(BASE64.encode(login_app_user.password.as_bytes())))
            .first::<AppUser>(connection)
            .map_err(|_| { ElogError::ObjectNotFound(login_app_user.username.clone()) });

        logged_app_user.and_then(Claims::create_token)
    }

    pub fn logout(connection: &SqlConnection, string_token: &str) -> Result<usize, ElogError> {
        Claims::invalidate_token(connection, string_token)
    }

    pub fn get_app_user_data(connection: &SqlConnection, app_user_id: i16) -> Result<AppUser, ElogError> {
        app_user
            .filter(id.eq(app_user_id))
            .get_result(connection)
            .map_err(|_| { ElogError::ObjectNotFound(app_user_id.to_string()) })
    }
}

// Base64 Decode Sample
//println!("BASE64 decoded is {:?}", String::from_utf8_lossy(&BASE64.decode(b"SGVsbG8gd29ybGQ=").unwrap()));
