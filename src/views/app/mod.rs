use super::path::Path;
use actix_web::web;
mod content_loader;
mod items;
mod login;
mod logout;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let app_path = Path::new("/");

    app.route(&app_path.define(""), web::get().to(items::items));
    app.route(&app_path.define("login"), web::get().to(login::login));
    app.route(&app_path.define("logout"), web::get().to(logout::logout));
}
