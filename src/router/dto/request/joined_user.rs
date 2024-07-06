use serde::Deserialize;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
pub struct DrawItemDto {
    pub name: String,
    pub qty: i32,
}

#[derive(Deserialize, Debug)]
pub struct NewUserDto {
  pub name: String,
  pub tag: String,
  pub created_at: NaiveDateTime,
}