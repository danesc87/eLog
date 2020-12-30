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

use crate::models::pay_type::{
    PayType,
    NewPayType,
    PayTypeList
};

use crate::error_mapper::ElogError;
use crate::authentication::AuthorizationService;

#[post("/pay_type")]
pub async fn insert_pay_type (
    pool: web::Data<MySqlPool>,
    pay_type: web::Json<NewPayType>,
    _: AuthorizationService
) -> Result<HttpResponse, ElogError> {
    let connection = mysql_pool_handler(pool);
    PayType::insert(&connection.unwrap(), pay_type.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/pay_type")]
pub async fn get_all_pay_types(
    pool: web::Data<MySqlPool>,
    _: AuthorizationService
) -> HttpResponse {
    let connection = mysql_pool_handler(pool);
    HttpResponse::Ok().json(PayTypeList::get_list(&connection.unwrap()))
}
