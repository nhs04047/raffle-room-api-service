use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::{
  common::{
    structs::{
      BaseResponse,
      ErrorResponse
    }
  },
  router::dto::{
    response::draw::DrawDto
  }
};

#[get("/rooms/{room_id}/draws")]
pub async fn get_draws(
  req: HttpRequest,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  HttpResponse::Ok().json(BaseResponse {
    result_code: 200,
    result_msg: String::from("Ok"),
    result_data: Some({})
  })
}
