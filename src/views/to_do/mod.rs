use actix_web::web;
mod create;
use super::path::Path;

pub fn item_factory(app: &mut web::ServiceConfig) {
  let base_path: Path = Path {
    path_prefix: String::from("/item"),
  };

  app.route(
    &base_path.define_path("/create/{title}"),
    web::post().to(create::create),
  );
}
