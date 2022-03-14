// use serde_json::value::Value;
// use serde_json::Map;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::database_schema::to_do;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::item::Item;
// use crate::processes::process_input;
// use crate::state::read_file;
// use crate::to_do::to_do_factory;

use super::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    // FOR USE WITH JSON STATE
    // let state: Map<String, Value> = read_file("./state.json");

    let title_ref = to_do_item.title.as_str();
    let status = to_do_item.status.as_str();

    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&connection);

    // FOR USE WITH JSON STATE
    // match to_do_factory(status, title) {
    //     Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
    //     Ok(item) => process_input(item, "delete", &state),
    // }

    HttpResponse::Ok().json(return_state())
}
