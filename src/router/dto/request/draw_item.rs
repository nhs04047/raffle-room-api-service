use serde::Deserialize;
use chrono::NaiveDateTime;


#[derive(Deserialize, Debug)]
pub struct NewDrawItemsDto {
  pub name: String,
  pub seq: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime,
}