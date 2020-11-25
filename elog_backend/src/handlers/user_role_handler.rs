use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    get
};

use crate::config::{
    MySqlPool,
    mysql_pool_handler
};

use crate::models::user_role::{
    UserRole,
    UserRoleList
};

#[get("/user_roles")]
pub async fn get_user_roles(request: HttpRequest, pool: web::Data<MySqlPool>) -> HttpResponse {
    println!("{:?}", request);
    let connection = mysql_pool_handler(pool);
    HttpResponse::Ok().json(UserRoleList::list(&connection.unwrap()))
}

#[get("/user_roles/{id}")]
pub async fn get_user_role_with_id(request: HttpRequest, pool: web::Data<MySqlPool>, path: web::Path<(i8,)>) -> HttpResponse {
    println!("{:?}", request);
    let connection = mysql_pool_handler(pool);
    HttpResponse::Ok().json(UserRole::get_with_id(&connection.unwrap(), path.into_inner().0))
}
