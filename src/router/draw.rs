use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::{
  common::{
    structs::{
      BaseResponse,
      ErrorResponse
    }
  },
  service::draw as DrawService
};

#[get("/rooms/{room_id}/draws")]
pub async fn get_draws(
  req: HttpRequest,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  let room_id = path.into_inner();
  let result = DrawService::get_draws_by_room_id(data.as_ref(), room_id).await;

  match result {
    Ok(v) => {
      HttpResponse::Ok().json(BaseResponse {
        result_code: 200,
        result_msg: String::from("Ok"),
        result_data: Some(v)
      })
    },
    Err(e) => {
      HttpResponse::InternalServerError().json(BaseResponse::<()> {
        result_code: 500,
        result_msg: format!("Error get rooms: {}", e),
        result_data: None,
      })
    }
  }
}
