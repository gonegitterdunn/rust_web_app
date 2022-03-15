mod app;
mod auth;
mod path;
mod to_do;
mod users;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    app::app_factory(app);
    auth::auth_factory(app);
    to_do::to_do_item_factory(app);
    users::user_factory(app);
}
