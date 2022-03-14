// extern crate bcrypt;

use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

use crate::database_schema::users;

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
  pub username: String,
  pub email: String,
  pub password: String,
  pub uuid: String,
}

impl NewUser {
  pub fn new(username: &str, email: &str, password: &str) -> Self {
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let uuid = Uuid::new_v4().to_string();

    Self {
      username: username.to_string(),
      email: email.to_string(),
      password: hashed_password.to_string(),
      uuid,
    }
  }
}
