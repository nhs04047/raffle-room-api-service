use actix_web::{get, HttpResponse};
// use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct DrawItem {
    name: String,
    qty: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Room {
    id: i32,
    name: String,
    password: String,
    set_draw_include_owner: i32,
    set_draw_order: String,
    draw_items: Vec<DrawItem>
}

#[get("/rooms")]
pub async fn get_rooms() -> HttpResponse {
    
    // 데이터베이스에서 Room 리스트를 가져오는 로직
    HttpResponse::Ok().json({}) // Placeholder
}
