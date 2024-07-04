use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
// use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct NewRoom {
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: i32,
  pub set_draw_order: String,
  pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRoom {
  pub name: Option<String>,
  pub password: Option<String>,
  pub set_draw_include_owner: Option<i32>,
  pub set_draw_order: Option<String>,
}
#[post("/rooms")]
pub async fn create_rooms(
  req: HttpRequest,
  body: web::Json<NewRoom>,
  data: web::Data<DatabaseConnection>,
) -> impl Responder {
  HttpResponse::Created().json({})
}

#[get("/rooms")]
pub async fn get_rooms(
  req: HttpRequest,
  data: web::Data<DatabaseConnection>,
) ->  impl Responder {
  HttpResponse::Ok().json({})
}

#[get("/rooms/{room_id}")]
pub async fn get_room
(
  req: HttpRequest,
  path: web::Path<i32>,
  data: web::Data<DatabaseConnection>
) -> impl Responder {
  HttpResponse::Ok().json({})
}

#[get("/rooms/{room_id}")]
pub async fn update_room(
  req: HttpRequest,
  path: web::Path<i32>,
  body: web::Json<UpdateRoom>,
  data: web::Data<DatabaseConnection>
) -> impl Responder {
  HttpResponse::Ok().json({})
}