use serde_json::value::Value;
use serde_json::Map;

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;
use super::to_do::structs::traits::get::Get;
use super::to_do::ItemTypes;

pub fn process_pending(item: Pending, command: &str, state: &Map<String, Value>) {
  let mut state = state.clone();

  match command {
    "get" => item.get(&item.base_struct.title, &state),
    "create" => item.create(
      &item.base_struct.title,
      &item.base_struct.status,
      &mut state,
    ),
    "delete" => item.delete(&item.base_struct.title, &mut state),
    "edit" => item.set_to_done(&item.base_struct.title, &mut state),
    _ => println!("command: {} not supported", command),
  }
}

pub fn process_done(item: Done, command: &str, state: &Map<String, Value>) {
  let mut state = state.clone();
  match command {
    "get" => item.get(&item.base_struct.title, &state),
    "delete" => item.delete(&item.base_struct.title, &mut state),
    "edit" => item.set_to_pending(&item.base_struct.title, &mut state),
    _ => println!("command: {} not supported", command),
  }
}

pub fn process_input(item: ItemTypes, command: &str, state: &Map<String, Value>) {
  match item {
    ItemTypes::Pending(item) => process_pending(item, command, state),
    ItemTypes::Done(item) => process_done(item, command, state),
  }
}
