// use serde_json::value::Value;
// use serde_json::Map;
// use crate::state::read_file;

use crate::diesel;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::to_do::to_do_factory;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::database_schema::to_do;
use crate::models::item::item::Item;

pub fn return_state() -> ToDoItems {
    // FOR USE WITH JSON STATE
    // let state: Map<String, Value> = read_file("./state.json");

    // FOR USE WITH DB
    let connection = establish_connection();

    // FOR USE WITH DB
    let items = to_do::table
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection) // Struct (Item) passed in load must
        .unwrap(); // implement 'Queryable' trait

    let mut array_buffer = Vec::new();

    // FOR USE WITH JSON STATE
    // Go through existing state file, creating ItemTypes objects
    // for each entry
    // for (key, value) in state {
    //   let item_type = value.as_str().unwrap();
    //   let item: ItemTypes = to_do_factory(item_type, key.as_str()).unwrap();

    //   array_buffer.push(item);
    // }

    // FOR USE WITH DB
    for item in items {
        let item = to_do_factory(&item.status, item.title.as_str()).unwrap();
        array_buffer.push(item);
    }

    // ToDoItems is serialized and implements Responder
    ToDoItems::new(array_buffer)
}
