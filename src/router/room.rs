use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use log::info;

use crate::{
  common::structs::{ErrorData, BaseResponse, ErrorResponse},
  domain::mapper::{Mapper, MapperWithId},
  router::dto::request::room::{NewRoomDto, UpdateRoomDto},
  service::{room as RoomService, structs as SerivceStructs}
};

use sea_orm::DatabaseConnection;

#[post("/rooms")]
pub async fn create_rooms(
  _req: HttpRequest,
  body: web::Json<NewRoomDto>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  info!("Attempting to parse JSON data: {}", body.created_at);
  let result = RoomService::create_room(
    data.get_ref(),
    SerivceStructs::NewRoom::map(&body.0)
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
      HttpResponse::InternalServerError().json(ErrorResponse::<&NewRoomDto> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error get room: {}", e)
          },
          result_data: Some(&body.0),
      })
    },
  }
}

#[get("/rooms")]
pub async fn get_rooms(
  _req: HttpRequest,
  data: web::Data<DatabaseConnection>,
) ->  impl Responder {
  let result = RoomService::get_rooms(data.get_ref()).await;

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
            error_msg: format!("Error get room: {}", e)
          },
          result_data: None,
      })
   },
  }
}

#[get("/rooms/{room_id}")]
pub async fn get_room
(
  _req: HttpRequest,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>
) -> impl Responder {
  let room_id = path.into_inner();
  let result = RoomService::get_room(data.get_ref(), room_id).await;

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
        result_data: Some(room_id),
        error_data: ErrorData {
          error_code: 400,
          error_msg: "Room not found".to_string()
        }
      })
    },
    Err(e) => {
      HttpResponse::InternalServerError().json(ErrorResponse::<i32> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error get room: {}", e)
          },
          result_data: None,
      })
    },
  }
}

#[get("/rooms/{room_id}")]
pub async fn update_room(
  _req: HttpRequest,
  path: web::Path<i32>,
  body: web::Json<UpdateRoomDto>,
  data: web::Data<DatabaseConnection>
) -> impl Responder {
  let result = RoomService::update_room(
    data.get_ref(),
    SerivceStructs::UpdateRoom::map_with_id(path.abs(), &body.0)
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
      HttpResponse::InternalServerError().json(ErrorResponse::<&UpdateRoomDto> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error update room: {}", e)
          },
          result_data: Some(&body.0),
      })
    },
  }
}