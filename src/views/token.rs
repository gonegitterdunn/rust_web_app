use actix_web::dev::ServiceRequest;

pub fn process_token(req: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(req) {
        Ok(token) => check_password(token.as_str()),
        Err(message) => Err(message),
    }
}

fn check_password(password: &str) -> Result<String, &'static str> {
    if password == "token" {
        Ok(password.to_string())
    } else {
        Err("token not authorized")
    }
}

fn extract_header_token(req: &ServiceRequest) -> Result<String, &'static str> {
    match req.headers().get("user-token") {
        Some(token) => match token.to_str() {
            Ok(processed_password) => Ok(processed_password.to_string()),
            Err(_processed_password) => Err("There was an error processing the token"),
        },
        None => Err("There was no token passed"),
    }
}
