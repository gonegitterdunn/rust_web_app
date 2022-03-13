use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use super::utils::return_state;

use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let title_reference: &String = &to_do_item.title.clone();
    let title: String = to_do_item.title.clone();

    let status = match &state.get(title_reference) {
        Some(result) => result.to_string().replace('\"', ""),
        None => return HttpResponse::NotFound().json(format!("{} not in state", title_reference)),
    };

    // if item status hasn't changed from what it was when it was passed into the view / created
    if status == to_do_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    match to_do_factory(&status, title.as_str()) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, "edit", &state),
    }

    HttpResponse::Ok().json(return_state())
}
