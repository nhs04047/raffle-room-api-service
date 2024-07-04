use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

#[get("/rooms/{room_id}/draws")]
pub async fn get_draws(
  req: HttpRequest,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
    HttpResponse::Ok().json({})
}
