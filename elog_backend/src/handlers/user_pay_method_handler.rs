use actix_web::{
    HttpResponse,
    web,
    get,
    post
};

use crate::config::{
    MySqlPool,
    mysql_pool_handler
};

use crate::models::user_pay_method::{
    UserPayMethod,
    NewUserPayMethod,
    UserPayMethodList
};

use crate::error_handler::ElogError;

#[post("/user_pay_method/{user_id}/{pay_type_id}")]
pub async fn insert_user_pay_method (
    pool: web::Data<MySqlPool>,
    path: web::Path<(i16, i8)>,
    mut new_user_pay_method: web::Json<NewUserPayMethod>
) -> Result<HttpResponse, ElogError>  {
    let connection = mysql_pool_handler(pool);
    let unwraped_path = path.into_inner();
    new_user_pay_method.user_id = unwraped_path.0;
    new_user_pay_method.pay_type_id = unwraped_path.1;
    UserPayMethod::insert(&connection.unwrap(), new_user_pay_method.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/user_pay_method")]
pub async fn get_all_user_pay_methods(pool: web::Data<MySqlPool>) -> HttpResponse {
    let connection = mysql_pool_handler(pool);
    HttpResponse::Ok().json(UserPayMethodList::get_list(&connection.unwrap()))
}
