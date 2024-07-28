use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::{
  common::structs::{BaseResponse, ErrorData, ErrorResponse},
  domain::mapper::MapperWithId,
  router::dto::request::draw_item::NewDrawItemsDto,
  service::{draw_item as DrawItemService, structs as SerivceStructs}
};

#[post("/rooms/{rooms_id}/draw-items")]
pub async fn create_draw_items(
  _req: HttpRequest,
  path: web::Path<i32>,
  body: web::Json<Vec<NewDrawItemsDto>>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  let room_id = path.into_inner();
  let result = DrawItemService::create_draw_items(
    data.get_ref(), 
    body.iter().map(|dto| SerivceStructs::NewDrawItem::map_with_id(room_id, &dto)).collect() 
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
      HttpResponse::InternalServerError().json(ErrorResponse::<&Vec<NewDrawItemsDto>> {
          result_code: 500,
          result_msg: format!("Unexpected Error"),
          error_data: ErrorData {
            error_code: 500,
            error_msg: format!("Error create user: {}", e)
          },
          result_data: Some(&body.0),
      })
    },
  }
}

#[get("/rooms/{rooms_id}/draw-items")]
pub async fn get_draw_items(
  _req: HttpRequest,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  let room_id = path.into_inner();
  let result = DrawItemService::get_darw_items_room_id(data.as_ref(), room_id).await;

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
            error_msg: format!("Error get draw items user: {}", e)
          },
          result_data: Some(room_id),
      })
    },
  }
}