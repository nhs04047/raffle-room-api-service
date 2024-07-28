use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;


#[derive(Deserialize, Serialize, Debug)]
pub struct NewDrawItemsDto {
  pub name: String,
  pub seq: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime,
}