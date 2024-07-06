use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Serialize, Debug)]
pub struct RoomDto {
  pub id: i32,
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: i32,
  pub set_draw_order: String,
  pub status: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}