use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    update,
    delete,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use crate::utils::error_mapper::ElogError;

use super::schema::user_category;
use super::schema::user_category::dsl::*;

// This Struct is only for showing data on endpoint
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserCategory {
    pub id: i16,
    pub category: String,
    pub description: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[serde(default)]
#[table_name = "user_category"]
pub struct NewUserCategory {
    pub user_id: i16,
    pub category: String,
    pub description: String
}

// Default implementation, lets send JSON body without user_id
impl Default for NewUserCategory {
    fn default() -> Self {
        NewUserCategory {
            user_id: 0,
            category: String::from(""),
            description: String::from("")
        }
    }
}

impl UserCategory {

    pub fn insert(
        connection: &SqlConnection,
        new_user_category: NewUserCategory
    ) -> Result<usize, ElogError> {
        insert_into(user_category)
            .values(&new_user_category)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_list(
        connection: &SqlConnection,
        logged_user_id: i16
    ) -> Result<Vec<UserCategory>, ElogError> {
        user_category
            .filter(user_id.eq(logged_user_id))
            .select((
                user_category::id,
                user_category::category,
                user_category::description
            ))
            .load::<UserCategory>(connection)
            .map_err(|error| { ElogError::ObjectNotFound(error.to_string()) })
    }

    pub fn update(
        connection: &SqlConnection,
        user_category_id: i16,
        new_user_category: NewUserCategory
    ) -> Result<usize, ElogError> {
        update(user_category.filter(id.eq(user_category_id)))
            .set((
                category.eq(new_user_category.category),
                description.eq(new_user_category.description)
            ))
            .execute(connection)
            .map_err(|error| { ElogError::ObjectNotFound(error.to_string()) })
    }

    pub fn delete_by_id(connection: &SqlConnection, user_category_id: i16) -> Result<usize, ElogError> {
        delete(user_category.filter(id.eq(user_category_id)))
            .execute(connection)
            .map_err(|error| { ElogError::ObjectNotFound(error.to_string()) })
    }
}
