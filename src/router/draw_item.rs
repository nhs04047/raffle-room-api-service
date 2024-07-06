use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
  common::{
    structs::{
      BaseResponse,
      // ErrorResponse
    }
  },
  router::dto::{
    request::draw_item::NewDrawItemsDto,
    response::draw_item::DrawItemsDto
  }
};

#[post("/rooms/{rooms_id}/draw-items")]
pub async fn create_draw_items(
  req: HttpRequest,
  path: web::Path<i32>,
  body: web::Json<NewDrawItemsDto>
) -> impl Responder {
  HttpResponse::Created().json(BaseResponse {
    result_code: 202,
    result_msg: String::from("Created"),
    result_data: Some({})
  })
}

#[get("/rooms/{rooms_id}/draw-items")]
pub async fn get_draw_items(
  req: HttpRequest,
  path: web::Path<i32>,
) -> impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Created"),
    result_data: Some({})
  })}