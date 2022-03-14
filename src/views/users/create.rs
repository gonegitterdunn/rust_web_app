use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::data_models::user::new_user::NewUser;
use crate::database::establish_connection;
use crate::database_schema::users;
use crate::diesel;
use crate::json_serialization::new_user::NewUserSchema;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
  let connection = establish_connection();

  let name = new_user.name.as_str();
  let email = new_user.email.as_str();
  let password = new_user.password.as_str();

  let new_user = NewUser::new(name, email, password);

  let insert_result = diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&connection);

  match insert_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap(),
  }
}
