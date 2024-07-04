use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
// use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct NewDrawItems {
  name: String,
  seq: i32,
  qty: i32,
  created_at: NaiveDateTime,
}

#[post("/rooms/{rooms_id}/draw-items")]
pub async fn create_draw_items(
  req: HttpRequest,
  path: web::Path<i32>,
  body: web::Json<NewDrawItems>
) -> impl Responder {
    HttpResponse::Created().json({})
}

#[get("/rooms/{rooms_id}/draw-items")]
pub async fn get_draw_items(
  req: HttpRequest,
  path: web::Path<i32>,
) -> impl Responder {
    HttpResponse::Ok().json({})
}