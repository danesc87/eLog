use actix_web::{
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

use crate::error_handler::ElogError;

#[post("/register")]
pub async fn register(
    pool: web::Data<MySqlPool>,
    app_user: web::Json<NewAppUser>
) -> Result<HttpResponse, ElogError> {
    let connection = mysql_pool_handler(pool);
    AppUser::register(&connection.unwrap(), app_user.0).map(|_| {
        HttpResponse::Created().finish()
    })
}

// Sample for Encoding and Decoding Strings with BASE64

// fn base64_stuff() {
//     use data_encoding::BASE64;
//
//     let encoded = BASE64.encode(b"Hello world");
//     println!("BASE64 encoded is {}", encoded);
//     println!("BASE64 decoded is {:?}", String::from_utf8_lossy(&BASE64.decode(b"SGVsbG8gd29ybGQ=").unwrap()));
// }
