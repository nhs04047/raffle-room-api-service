use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserDto {
  pub name: String,
  pub created_at: NaiveDateTime,
}