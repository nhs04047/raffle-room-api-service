use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Serialize, Debug)]
pub struct UserDto {
  pub id: i32,
  pub name: String,
  pub tag: String,
  pub room_id: i32,
  pub created_at: NaiveDateTime,
}