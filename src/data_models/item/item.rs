use crate::schema::to_do;

use super::super::user::user::User;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)] // needs the Associations macro
#[table_name = "to_do"]
pub struct Item {
  pub id: i32,
  pub title: String,
  pub status: String,
  pub user_id: i32,
}
