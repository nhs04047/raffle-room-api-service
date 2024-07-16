use serde::Deserialize;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
pub struct NewUserDto {
  pub name: String,
  pub created_at: NaiveDateTime,
}