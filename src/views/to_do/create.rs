use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do;

pub async fn create(req: HttpRequest) -> String {
  println!("in create");
  let state: Map<String, Value> = read_file("./state.json");

  let title: &str = req.match_info().get("title").unwrap();

  let title_reference = title;

  let item = to_do::to_do_factory("pending", title).expect("create ");

  // add the to do item from the state.json
  process_input(item, "create", &state);

  // return a message to viewer
  return format!("{} created", title_reference);
}
