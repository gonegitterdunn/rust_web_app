use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUserSchema {
  pub username: String,
  pub password: String,
  pub email: String,
}
