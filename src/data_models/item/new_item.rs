use crate::schema::to_do;

#[derive(Insertable)]
#[table_name = "to_do"]
pub struct NewItem {
  pub title: String,
  pub status: String,
  pub user_id: i32,
}

impl NewItem {
  pub fn new(title: &str, user_id: i32) -> Self {
    Self {
      title: title.to_string(),
      status: "pending".to_string(),
      user_id,
    }
  }
}
