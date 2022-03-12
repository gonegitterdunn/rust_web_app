use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use super::utils::return_state;

use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
  let state: Map<String, Value> = read_file("./state.json");

  let title = to_do_item.title.as_str();
  let status = to_do_item.status.as_str();

  match to_do_factory(status, title) {
    Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
    Ok(item) => process_input(item, "delete", &state),
  }
  HttpResponse::Ok().json(return_state())
}
