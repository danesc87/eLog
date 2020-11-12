use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    get,
    post
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ELogBasicResponse {
    response: String
}

// GET Request sample
#[get("/")]
pub async fn hello_world(request: HttpRequest) -> HttpResponse {
    println!("{:?}", request);
    let object = ELogBasicResponse { response: String::from("Hello World!") };
    HttpResponse::Ok().json(object)
}

// GET Resquest with dynamic URL sample
#[get("/user/{name}")]
pub async fn hello_user_name(request: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", request);
    let object = ELogBasicResponse { response: format!("Hello {}!", path.into_inner().0) };
    HttpResponse::Ok().json(object)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summable {
    a: i32,
    b: i32
}

// POST Request sample
#[post("/sum")]
pub async fn test_sum(request: HttpRequest, summable: web::Json<Summable>) -> HttpResponse {
    println!("{:?}", request);
    let object = ELogBasicResponse { response: format!("Sum of a + b = {}", (summable.a + summable.b).to_string()) };
    HttpResponse::Ok().json(object)
}