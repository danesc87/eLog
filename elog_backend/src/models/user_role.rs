use serde::{Deserialize, Serialize};
use diesel::{
    MysqlConnection,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use super::schema::user_role::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserRole {
    pub id: i8,
    pub description: String
}

impl UserRole {
    pub fn get_with_id(connection: &MysqlConnection, role_id: i8) -> Self {
        user_role
            .filter(id.eq(role_id))
            .first::<UserRole>(connection)
            .expect("User Role not found")
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserRoleList(pub Vec<UserRole>);

impl UserRoleList {
    pub fn list(connection: &MysqlConnection) -> Self {
        let result = user_role
            .load::<UserRole>(connection)
            .expect("Error loading User Roles");

        UserRoleList(result)
    }
}
