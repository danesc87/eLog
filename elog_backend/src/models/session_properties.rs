use serde::{Deserialize, Serialize};
use super::token::Claims;

use crate::utils::database_utils::SqlConnection;

extern crate serde_json;
use serde_json::{Value, Map};

#[derive(Serialize, Deserialize)]
pub struct SessionProperties {
    is_valid_token: bool,
    properties: Option<Map<String, Value>>
}

impl SessionProperties {

    pub fn get_session_properties(connection: &SqlConnection, token: &str) -> Self {
        let is_valid_token = Claims::is_valid_token(connection, token);
        if is_valid_token {
            let mut properties = Map::new();
            Self::add_properties(connection, token, &mut properties);
            SessionProperties {
                is_valid_token,
                properties: Some(properties)
            }
        } else {
            SessionProperties {
                is_valid_token,
                properties: None
            }
        }
    }

    pub fn no_token_session_properties() -> Self {
        SessionProperties {
            is_valid_token: false,
            properties: None
        }
    }

    fn add_properties(
        connection: &SqlConnection,
        token: &str,
        properties: &mut Map<String, Value>
    ) {
        properties.insert(String::from("version"), Value::String(String::from("0.99b")));
        properties.insert(String::from("userData"), Value::Object(Self::get_app_user_data(connection, token)));
    }

    fn get_app_user_data(connection: &SqlConnection, token: &str) -> Map<String, Value>  {
        use super::app_user::AppUser;
        let claims = Claims::decode_token(token).unwrap().claims;
        let app_user = AppUser::get_app_user_data(connection, claims.id).unwrap();
        let mut app_user_map = Map::new();
        app_user_map.insert(String::from("firstName"), Value::String(app_user.first_name));
        app_user_map.insert(String::from("lastName"), Value::String(app_user.last_name));
        app_user_map.insert(String::from("email"), Value::String(app_user.email));
        app_user_map.insert(String::from("userName"), Value::String(app_user.username));
        app_user_map
    }
}
