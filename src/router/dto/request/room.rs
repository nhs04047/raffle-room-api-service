use serde::Deserialize;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
pub struct NewRoomDto {
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: i32,
  pub set_draw_order: String,
  pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRoomDto {
  pub name: Option<String>,
  pub password: Option<String>,
  pub set_draw_include_owner: Option<i32>,
  pub set_draw_order: Option<String>,
}