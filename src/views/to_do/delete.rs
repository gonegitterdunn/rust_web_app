use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use to_do::dsl::*;

use crate::data_models::item::item::Item;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;

use super::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref = to_do_item.title.as_str();
    let connection = establish_connection();

    let items = to_do
        .filter(to_do::columns::title.eq(title_ref))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);

    HttpResponse::Ok().json(return_state())
}
