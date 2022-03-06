pub mod structs;
pub use structs::done::Done;
pub use structs::pending::Pending;

pub enum ItemTypes {
  Done(Done),
  Pending(Pending),
}

pub fn to_do_factory(input_type: &str, input_title: &str) -> Result<ItemTypes, &'static str> {
  if input_type == "done" {
    let done = Done::new(input_title);
    Ok(ItemTypes::Done(done))
  } else if input_type == "pending" {
    let pending = Pending::new(input_title);
    Ok(ItemTypes::Pending(pending))
  } else {
    Err("unaccepted type")
  }
}
