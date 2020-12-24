use serde::{Deserialize, Serialize};
use jsonwebtoken::{
    encode,
    Header,
    EncodingKey
};

use super::app_user::{LoginAppUser, LoggedAppUser};
use crate::config::get_secret_key;
use crate::error_mapper::ElogError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    username: String,
    iat: i64,
    exp: i64
}

impl Claims {
    pub fn create_token(logged_app_user: LoginAppUser) -> Result<LoggedAppUser, ElogError> {
        let claims = Self::with_app_user(&logged_app_user);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret_key().as_ref())
        )
        .map_err(|_| { ElogError::TokenCreationError });
        Ok(LoggedAppUser {
            token_type: "Bearer".into(),
            access_token: token.unwrap()
        })
    }

    fn with_app_user(logged_app_user: &LoginAppUser) -> Self {
        use chrono::{Local, Duration};

        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            username: logged_app_user.username.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::minutes(60)).timestamp()
        }
    }
}
