use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    post
};

use crate::config::{
    MySqlPool,
    mysql_pool_handler
};

use crate::models::app_user::{
    AppUser,
    NewAppUser
};

#[post("/register")]
pub async fn register(request: HttpRequest, pool: web::Data<MySqlPool>, app_user: web::Json<NewAppUser>) -> HttpResponse {
    println!("{:?}", request);
    println!("{:?}", app_user);
    let connection = mysql_pool_handler(pool);
    HttpResponse::Created().json(AppUser::register(&connection.unwrap(), app_user.0))
}

// Sample for Encoding and Decoding Strings with BASE64

// fn base64_stuff() {
//     use data_encoding::BASE64;
//
//     let encoded = BASE64.encode(b"Hello world");
//     println!("BASE64 encoded is {}", encoded);
//     println!("BASE64 decoded is {:?}", String::from_utf8_lossy(&BASE64.decode(b"SGVsbG8gd29ybGQ=").unwrap()));
// }
