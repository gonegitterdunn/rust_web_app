use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;
use super::to_do::structs::traits::get::Get;
use super::to_do::ItemTypes;
use serde_json::value::Value;
use serde_json::Map;

fn process_pending(item: Pending, command: &str, state: &Map<String, Value>) {
  let mut state = state.clone();

  let title = &item.super_struct.title;
  let status = &item.super_struct.status;
  let mut_state = &mut state;

  match command {
    "create" => item.create(title, status, mut_state),
    "edit" => item.set_to_done(title, mut_state),
    "delete" => item.delete(title, mut_state),
    "get" => item.get(title, &state),
    _ => println!("command: {} not supported", command),
  }
}

fn process_done(item: Done, command: &str, state: &Map<String, Value>) {
  let mut state = state.clone();

  let title = &item.super_struct.title;
  let mut_state = &mut state;

  match command {
    "edit" => item.set_to_done(title, mut_state),
    "delete" => item.delete(title, mut_state),
    "get" => item.get(title, &state),
    _ => println!("command: {} not supported", command),
  }
}

pub fn process_input(item: ItemTypes, command: &str, state: &Map<String, Value>) {
  match item {
    ItemTypes::Pending(item) => process_pending(item, command, state),
    ItemTypes::Done(item) => process_done(item, command, state),
  }
}
