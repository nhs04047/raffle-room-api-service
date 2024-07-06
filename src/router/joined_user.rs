use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::{
  common::{
    structs::{
      BaseResponse,
      ErrorResponse
    }
  },
  router::dto::{
    request::joined_user::NewUserDto,
    response::joined_user::UserDto
  }
};

#[post("/rooms/{room_id}/users")]
pub async fn create_user(
  req: HttpRequest,
  body: web::Json<NewUserDto>,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  HttpResponse::Created().json(BaseResponse {
    result_code: 202,
    result_msg: String::from("Created"),
    result_data: Some({})
  })
}

#[get("/rooms/{room_id}/users")]
pub async fn get_users(
  req: HttpRequest,
  body: web::Json<NewUserDto>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Ok"),
    result_data: Some({})
  })
}

#[get("/rooms/{room_id}/users/{user_id}")]
pub async fn get_user(
  req: HttpRequest,
  path: web::Path<(i32, i32)>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Ok"),
    result_data: Some({})
  })
}


