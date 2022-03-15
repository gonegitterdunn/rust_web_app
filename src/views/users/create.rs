use crate::data_models::user::new_user::NewUser;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::new_user::NewUserSchema;
use crate::schema::users;

use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
  let connection = establish_connection();

  let username = new_user.username.clone();
  let password = new_user.password.clone();
  let email = new_user.email.clone();

  let new_user = NewUser::new(username, password, email);

  let insert_result = diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&connection);

  match insert_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap(),
  }
}
