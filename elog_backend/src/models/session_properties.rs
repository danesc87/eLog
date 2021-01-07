use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use diesel::MysqlConnection;

use super::token::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionProperties {
    is_valid_token: bool,
    properties: Option<HashMap<String, String>>
}

impl SessionProperties {

    pub fn get_session_properties(connection: &MysqlConnection, token: &str) -> Self {
        let is_valid_token = Claims::is_valid_token(connection, token);
        if is_valid_token {
            //todo!(This HashMap should be mut when we start add properties)
            let properties = HashMap::new();
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
}
