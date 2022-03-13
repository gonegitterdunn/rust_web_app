use super::path::Path;
use actix_web::web;
mod content_loader;
mod items;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let app_path = Path::new("/");

    app.route(&app_path.define(""), web::get().to(items::items));
}
