use super::jwt;
use actix_web::dev::ServiceRequest;

pub fn check_password(password: &str) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password.to_string()) {
        Ok(_token) => Ok(String::from("passed")),
        Err(message) => Err(message),
    }
}

pub fn extract_header_token(req: &ServiceRequest) -> Result<String, &'static str> {
    match req.headers().get("user-token") {
        Some(token) => match token.to_str() {
            Ok(processed_password) => Ok(processed_password.to_string()),
            Err(_processed_password) => Err("There was an error processing the token"),
        },
        None => Err("There was no token passed"),
    }
}
