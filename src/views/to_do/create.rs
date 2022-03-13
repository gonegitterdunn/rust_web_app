// use serde_json::value::Value;
// use serde_json::Map;

use actix_web::{HttpRequest, Responder};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::database_schema::to_do;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
// use crate::processes::process_input;
// use crate::state::read_file;
// use crate::to_do;

use super::utils::return_state;

pub async fn create(req: HttpRequest) -> impl Responder {
  // FOR USE WITH JSON STATE
  // let state: Map<String, Value> = read_file("./state.json");

  let title = req.match_info().get("title").unwrap();

  let connection = establish_connection();

  // FOR USE WITH JSON STATE
  // let item = to_do::to_do_factory("pending", title).expect("create ");

  let items = to_do::table
    .filter(to_do::columns::title.eq(title))
    .order(to_do::columns::id.asc())
    .load::<Item>(&connection)
    .unwrap();

  if items.is_empty() {
    let new_post = NewItem::new(title.to_string());
    let _ = diesel::insert_into(to_do::table)
      .values(&new_post)
      .execute(&connection);
  }

  // FOR USE WITH JSON STATE
  // add the to do item from the state.json
  // process_input(item, "create", &state);

  // return a message to viewer
  return_state()
}
