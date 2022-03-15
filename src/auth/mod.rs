pub mod jwt;
mod processes;

use actix_web::dev::ServiceRequest;

pub fn process_token(req: &ServiceRequest) -> Result<String, &'static str> {
  match processes::extract_header_token(req) {
    Ok(token) => match processes::check_password(token.as_str()) {
      Ok(token) => Ok(token),
      Err(message) => Err(message),
    },
    Err(message) => Err(message),
  }
}
