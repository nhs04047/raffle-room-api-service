use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use sea_orm::DatabaseConnection;

use crate::{
  common::structs::{
      BaseResponse,
      // ErrorResponse
    },
  router::dto::{
    request::room::{NewRoomDto, UpdateRoomDto},
    response::room::RoomDto
  }
};


#[post("/rooms")]
pub async fn create_rooms(
  req: HttpRequest,
  body: web::Json<NewRoomDto>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  HttpResponse::Created().json(BaseResponse {
    result_code: 202,
    result_msg: String::from("Created"),
    result_data: Some({})
  })
}

#[get("/rooms")]
pub async fn get_rooms(
  req: HttpRequest,
  data: web::Data<DatabaseConnection>,
) ->  impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Ok"),
    result_data: Some({})
  })
}

#[get("/rooms/{room_id}")]
pub async fn get_room
(
  req: HttpRequest,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>
) -> impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Ok"),
    result_data: Some({})
  })
}

#[get("/rooms/{room_id}")]
pub async fn update_room(
  req: HttpRequest,
  path: web::Path<i32>,
  body: web::Json<UpdateRoomDto>,
  data: web::Data<DatabaseConnection>
) -> impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Ok"),
    result_data: Some({})
  })
}