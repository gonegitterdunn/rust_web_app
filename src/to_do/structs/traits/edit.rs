use crate::state::write_to_file;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub trait Edit {
  fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
    state.insert(title.to_string(), json!("done"));

    write_to_file("./state.json", state);

    println!("\n\n{} is being set to done...\n\n", title);
  }

  fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
    state.insert(title.to_string(), json!("pending"));

    write_to_file("./state.json", state);

    println!("\n\n{} is being set to pending...\n\n", title);
  }
}
