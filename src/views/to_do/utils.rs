use crate::diesel;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::to_do::to_do_factory;
use diesel::prelude::*;
use to_do::dsl::*;

use crate::data_models::item::item::Item;
use crate::database::establish_connection;
use crate::database_schema::to_do;

pub fn return_state() -> ToDoItems {
    let connection = establish_connection();

    let items = to_do
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    let mut array_buffer = Vec::new();

    for item in items {
        let _item = to_do_factory(item.status.as_str(), item.title.as_str()).unwrap();
        array_buffer.push(_item);
    }

    // ToDoItems is serialized and implements Responder
    ToDoItems::new(array_buffer)
}
