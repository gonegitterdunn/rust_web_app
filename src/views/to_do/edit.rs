use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use to_do::dsl::*;

use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;

use super::utils::return_state;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();

    let connection = establish_connection();
    let results = to_do.filter(to_do::columns::title.eq(title_ref));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&connection);

    HttpResponse::Ok().json(return_state())
}
