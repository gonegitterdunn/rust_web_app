// extern crate bcrypt;

use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

use crate::schema::users;

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
  pub username: String,
  pub password: String,
  pub email: String,
  pub unique_id: String,
}

impl NewUser {
  pub fn new(username: String, password: String, email: String) -> Self {
    let hashed_password = hash(password.as_str(), DEFAULT_COST).unwrap();
    let uuid = Uuid::new_v4().to_string();

    Self {
      username,
      password: hashed_password.to_string(),
      email,
      unique_id: uuid,
    }
  }
}
