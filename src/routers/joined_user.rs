use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct DrawItem {
    name: String,
    qty: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewUser {
  pub name: String,
  pub tag: String,
  pub created_at: NaiveDateTime,
}

#[post("/rooms/{room_id}/users")]
pub async fn create_user(
  req: HttpRequest,
  body: web::Json<NewUser>,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
    HttpResponse::Created().json({})
}

#[get("/rooms/{room_id}/users")]
pub async fn get_users(
  req: HttpRequest,
  body: web::Json<NewUser>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
    HttpResponse::Ok().json({})
}

#[get("/rooms/{room_id}/users/{user_id}")]
pub async fn get_user(
  req: HttpRequest,
  body: web::Json<NewUser>,
  path: web::Path<(i32, i32)>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
    HttpResponse::Ok().json({})
}


