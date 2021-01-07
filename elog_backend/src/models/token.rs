use serde::{Deserialize, Serialize};
use chrono::{
    NaiveDateTime,
    Duration
};
use super::app_user::{AppUser, AppUserToken};
use diesel::{
    MysqlConnection,
    QueryDsl,
    insert_into,
    RunQueryDsl,
    ExpressionMethods
};
use jsonwebtoken::{
    encode,
    decode,
    Header,
    Algorithm,
    EncodingKey,
    DecodingKey,
    Validation,
    TokenData
};
use super::schema::invalid_tokens;
use super::schema::invalid_tokens::dsl::*;


use crate::config::get_secret_key;
use crate::error_mapper::ElogError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i16,
    username: String,
    exp: i64
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "invalid_tokens"]
pub struct InvalidToken {
    pub string_token: String,
    pub expiration_date: NaiveDateTime
}

impl Claims {

    pub fn create_token(app_user: AppUser) -> Result<AppUserToken, ElogError> {
        let claims = Self::with_app_user(&app_user);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret_key().as_ref())
        )
        .map_err(|_| { ElogError::TokenCreationError });
        Ok(AppUserToken {
            token_type: "Bearer".into(),
            access_token: token.unwrap()
        })
    }

    fn with_app_user(app_user: &AppUser) -> Self {
        use chrono::Local;

        Claims {
            id: app_user.id,
            username: app_user.username.to_owned(),
            exp: (Local::now() + Duration::minutes(60)).timestamp()
        }
    }

    pub fn is_valid_token(connection: &MysqlConnection, token: &str) -> bool {
        let invalid_token = invalid_tokens
            .filter(string_token.eq(token.clone().to_owned()))
            .select(string_token)
            .first::<String>(connection)
            .map_err(|_| { ElogError::ObjectNotFound(token.clone().to_owned()) });

        // Returns upside down, if we found a token on DB means is an invalid one
        match invalid_token {
            Ok(_) => false,
            Err(_) => true
        }
    }

    pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(get_secret_key().as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
    }

    pub fn invalidate_token(connection: &MysqlConnection, token: &str) -> Result<usize, ElogError> {
        let claims_token = Self::decode_token(token).unwrap().claims;
        let exp_date = Duration::milliseconds(claims_token.exp);
        let invalid_token = InvalidToken {
            string_token: token.to_owned(),
            expiration_date: NaiveDateTime::from_timestamp(exp_date.num_milliseconds(), 0)
        };

        insert_into(invalid_tokens)
            .values(&invalid_token)
            .execute(connection)
            .map_err(|_| { ElogError::InsertFailure })
    }
}
