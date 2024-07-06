use serde::Serialize;
use chrono::NaiveDateTime;


#[derive(Serialize, Debug)]
pub struct DrawItemsDto {
  pub id: i32,
  pub name: String,
  pub seq: i32,
  pub room_id: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime,
}