use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::{
  common::structs::{BaseResponse, ErrorData, ErrorResponse},
  domain::mapper::MapperWithId, router::dto::request::joined_user::NewUserDto, service::{joined_user as JoinedUserService, structs as SerivceStructs}
};

#[post("/rooms/{room_id}/users")]
pub async fn create_user(
  _req: HttpRequest,
  body: web::Json<NewUserDto>,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  let room_id = path.into_inner();
  let result = JoinedUserService::create_user(
    data.get_ref(), 
    SerivceStructs::NewUser::map_with_id(room_id, &body.0)
  ).await;

  match result {
    Ok(v) => {
        HttpResponse::Created().json(BaseResponse {
            result_code: 201,
            result_msg: String::from("Created"),
            result_data: Some(v), 
        })
    },
    Err(e) => {
      HttpResponse::InternalServerError().json(ErrorResponse::<&NewUserDto> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error create user: {}", e)
          },
          result_data: Some(&body),
      })
    },
  }
}

#[get("/rooms/{room_id}/users")]
pub async fn get_users(
  _req: HttpRequest,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  let room_id = path.into_inner();
  let result = JoinedUserService::get_users_by_room_id(data.as_ref(), room_id).await;

  match result {
    Ok(v) => {
      HttpResponse::Ok().json(BaseResponse {
        result_code: 200,
        result_msg: String::from("Ok"),
        result_data: Some(v)
      })
    },
    Err(e) => {
      HttpResponse::InternalServerError().json(ErrorResponse::<i32> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error get users: {}", e)
          },
          result_data: Some(room_id),
      })
    },
  }
}

#[get("/rooms/{room_id}/users/{user_id}")]
pub async fn get_user(
  _req: HttpRequest,
  path: web::Path<(i32, i32)>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {

  let (_room_id, user_id) = path.into_inner();
  let result = JoinedUserService::get_user(data.as_ref(), user_id).await;

  match result {
    Ok(Some(v)) => {
      HttpResponse::Ok().json(BaseResponse {
        result_code: 200,
        result_msg: String::from("Ok"),
        result_data: Some(v)
      })
    },
    Ok(None) => {
      HttpResponse::NotFound().json(ErrorResponse::<i32> {
        result_code: 400,
        result_msg: String::from("NotFound"),
        result_data: Some(user_id),
        error_data: ErrorData {
          error_code: 400,
          error_msg: "Room not found".to_string()
        }
      })
    },
    Err(e) => {
      HttpResponse::InternalServerError().json(ErrorResponse::<(i32, i32)> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error get user: {}", e)
          },
          result_data: Some((_room_id, user_id)),
      })
    },
  }
}


