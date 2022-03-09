use super::path::Path;
use actix_web::web;
mod create;
mod edit;
mod get;
mod utils;

pub fn to_do_item_factory(app: &mut web::ServiceConfig) {
  let to_do_path = Path::new("/item");

  app.route(
    &to_do_path.define("/create/{title}"),
    web::post().to(create::create),
  );

  app.route(&to_do_path.define("/get"), web::get().to(get::get));

  app.route(&to_do_path.define("/edit"), web::put().to(edit::edit));
}
