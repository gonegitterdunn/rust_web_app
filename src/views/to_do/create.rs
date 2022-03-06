use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do;

pub async fn create(req: HttpRequest) -> String {
  let state: Map<String, Value> = read_file("./state.json");

  let title: String = req.match_info().get("title").unwrap().to_string();

  let title_reference: String = title.clone();

  let item = to_do::to_do_factory(&"pending", title.as_str()).expect("create ");

  process_input(item, "create", &state);
  format!("{} created", title_reference)
}
