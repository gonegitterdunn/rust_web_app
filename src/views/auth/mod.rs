mod login;
mod logout;

use super::path::Path;
use actix_web::web;

pub fn auth_factory(app: &mut web::ServiceConfig) {
  let auth_path = Path::new("/auth");
  
  app
    .route(&auth_path.define("/login"), web::get().to(login::login))
    .route(&auth_path.define("/logout"), web::get().to(logout::logout));
}
