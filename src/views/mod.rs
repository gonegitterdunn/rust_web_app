mod app;
mod auth;
mod path;
mod to_do;
pub mod token;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
  auth::auth_factory(app);
  to_do::to_do_item_factory(app);
  app::app_factory(app);
}
