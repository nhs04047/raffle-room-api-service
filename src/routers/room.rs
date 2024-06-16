use actix_web::{get, HttpResponse};
// use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
struct Room {
    id: i32,
    name: String,
    password: String,
    set_draw_include_owner: i32,
    set_draw_order: String,
    status: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[get("/rooms")]
pub async fn get_rooms() -> HttpResponse {
    // 데이터베이스에서 Room 리스트를 가져오는 로직
    HttpResponse::Ok().json({}) // Placeholder
}
