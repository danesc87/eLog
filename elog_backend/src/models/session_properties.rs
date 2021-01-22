use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;

use super::token::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionProperties {
    is_valid_token: bool,
    properties: Option<HashMap<String, String>>
}

impl SessionProperties {

    pub fn get_session_properties(connection: &SqlConnection, token: &str) -> Self {
        let is_valid_token = Claims::is_valid_token(connection, token);
        if is_valid_token {
            let mut properties = HashMap::new();
            Self::add_properties(&mut properties);
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

    fn add_properties(properties: &mut HashMap<String, String>) {
        properties.insert("version".to_string(), "0.99b".to_string());
    }
}
