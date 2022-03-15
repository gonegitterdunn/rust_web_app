use actix_web::{HttpRequest, Responder};
use diesel::prelude::*;
use to_do::dsl::*;

use crate::data_models::item::item::Item;
use crate::data_models::item::new_item::NewItem;
use crate::database::establish_connection;
use crate::schema::to_do;
use crate::diesel;

use super::utils::return_state;

pub async fn create(req: HttpRequest) -> impl Responder {
  let title_ref = req.match_info().get("title").unwrap();
  let connection = establish_connection();

  let items = to_do
    .filter(to_do::columns::title.eq(title_ref))
    .order(to_do::columns::id.asc())
    .load::<Item>(&connection)
    .unwrap();

  if items.is_empty() {
    let new_post = NewItem::new(title_ref, 1);
    let _ = diesel::insert_into(to_do)
      .values(&new_post)
      .execute(&connection);
  }

  // return a message to viewer
  return_state()
}
