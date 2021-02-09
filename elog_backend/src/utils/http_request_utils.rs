use actix_web::http::HeaderValue;

pub fn get_token_from_auth_header(auth: Option<&HeaderValue>) -> &str {
    let splitted_header_token: Vec<&str> = auth
        .unwrap()
        .to_str()
        .unwrap()
        .split("Bearer")
        .collect();
    splitted_header_token[1].trim()
}
