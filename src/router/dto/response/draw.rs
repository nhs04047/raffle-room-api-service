use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Serialize, Debug)]
pub struct DrawDto{
  pub id: i32,
  pub room_id: i32,
  pub user_id: i32,
  pub draw_item_id: i32,
  pub created_at: NaiveDateTime,
  
}