use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use std::fs;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> Map<String, Value> {
  let mut file = File::open(file_name).unwrap();
  let mut data = String::new();

  // std::io::Read nedded for File.read_to_string
  file.read_to_string(&mut data).unwrap();

  let json: Value = serde_json::from_str(&data).unwrap();
  let state: Map<String, Value> = json.as_object().unwrap().clone();
  state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
  fs::write(file_name, json!(state).to_string()).expect("unable to write file");
}
