
use chrono::NaiveDateTime;

use crate::domain::structs::{SetDrawIncludeOwnerFlag, SetDrawOrderFlag, RoomStatusFlag};

#[derive(Debug)]
pub struct NewRoom {
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: SetDrawIncludeOwnerFlag,
  pub set_draw_order: SetDrawOrderFlag,
  pub status: RoomStatusFlag,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

#[derive(Debug)]
pub struct UpdateRoom {
  pub id: i32,
  pub name: Option<String>,
  pub password: Option<String>,
  pub set_draw_include_owner: Option<SetDrawIncludeOwnerFlag>,
  pub set_draw_order: Option<SetDrawOrderFlag>,
  pub status: Option<RoomStatusFlag>,
  pub updated_at: NaiveDateTime
}

#[derive(Debug)]
pub struct Room {
  pub id: i32,
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: SetDrawIncludeOwnerFlag,
  pub set_draw_order: SetDrawOrderFlag,
  pub status: RoomStatusFlag,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}