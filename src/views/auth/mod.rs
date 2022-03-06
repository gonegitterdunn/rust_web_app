use super::path::Path;
use actix_web::web;
mod login;
mod logout;

pub fn auth_factory(app: &mut web::ServiceConfig) {
  let auth_path = Path {
    path_prefix: String::from("/auth"),
  };

  app
    .route(
      &auth_path.define_path("/login"),
      web::get().to(login::login),
    )
    .route(
      &auth_path.define_path("/logout"),
      web::get().to(logout::logout),
    );
}
