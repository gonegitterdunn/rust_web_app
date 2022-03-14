mod create;

use actix_web::web;

use super::path::Path;

pub fn user_factory(app: &mut web::ServiceConfig) {
  let user_path = Path::new("/user");

  app.route(&user_path.define("/create"), web::post().to(create::create));
}
