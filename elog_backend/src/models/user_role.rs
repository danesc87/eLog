use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use super::schema::user_role::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserRole {
    pub id: i16,
    pub description: String
}

impl UserRole {
    pub fn get_with_id(connection: &SqlConnection, role_id: i16) -> Self {
        user_role
            .filter(id.eq(role_id))
            .first::<UserRole>(connection)
            .expect("User Role not found")
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserRoleList(pub Vec<UserRole>);

impl UserRoleList {
    pub fn list(connection: &SqlConnection) -> Self {
        let result = user_role
            .load::<UserRole>(connection)
            .expect("Error loading User Roles");

        UserRoleList(result)
    }
}
