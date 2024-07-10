use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
  service::{room as RoomService, structs as RoomStructs}
};

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
  let result = RoomService::create_room(
    data.get_ref(),
    RoomStructs::NewRoom::from_dto(body.0)
  ).await;

  match result {
    Ok(_) => {
        HttpResponse::Created().json(BaseResponse {
            result_code: 201,
            result_msg: String::from("Created"),
            result_data: Some({}),
        })
    },
    Err(e) => {
        HttpResponse::InternalServerError().json(BaseResponse::<()> {
            result_code: 500,
            result_msg: format!("Error creating room: {}", e),
            result_data: None,
        })
    },
  }
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