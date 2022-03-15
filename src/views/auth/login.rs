use crate::auth::jwt::JwtToken;
use crate::data_models::user::user::User;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::login::Login;
use crate::schema::users;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let _username = credentials.username.clone();
    let password = credentials.password.clone();

    let connection = establish_connection();
    let users = users::table
        .filter(users::columns::username.eq(_username.as_str()))
        .load::<User>(&connection)
        .unwrap();

    if users.is_empty() {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].clone().verify(password.as_str()) {
        true => {
            let token: String = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok().header("token", token).await.unwrap()
        }
        false => HttpResponse::Unauthorized().await.unwrap(),
    }
}
