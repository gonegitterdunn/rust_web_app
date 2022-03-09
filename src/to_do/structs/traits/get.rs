use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
  fn get(&self, title: &str, state: &Map<String, Value>) {
    let value: Option<&Value> = state.get(title);

    match value {
      Some(result) => {
        println!("\n\nItem: {}\n\n", title);
        println!("Status: {}\n\n", result);
      }
      None => println!("Item: {} wasn't found", title),
    }
  }
}
