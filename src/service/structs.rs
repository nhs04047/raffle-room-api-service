
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::domain::structs::{SetDrawIncludeOwnerFlag, SetDrawOrderFlag, RoomStatusFlag};

#[derive(Debug, Serialize)]
pub struct NewRoom {
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: SetDrawIncludeOwnerFlag,
  pub set_draw_order: SetDrawOrderFlag,
  pub status: RoomStatusFlag,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

#[derive(Debug, Serialize)]
pub struct UpdateRoom {
  pub id: i32,
  pub name: Option<String>,
  pub password: Option<String>,
  pub set_draw_include_owner: Option<SetDrawIncludeOwnerFlag>,
  pub set_draw_order: Option<SetDrawOrderFlag>,
  pub status: Option<RoomStatusFlag>,
  pub updated_at: NaiveDateTime
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct NewUser {
 pub name: String,
 pub room_id: i32,
 pub tag: String,
 pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub tag: String,
  pub room_id: i32,
  pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct DrawItem {
  pub id: i32,
  pub room_id: i32,
  pub name: String,
  pub seq: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize)]
pub struct NewDrawItem {
  pub room_id: i32,
  pub name: String,
  pub seq: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Draw {
  pub id: i32,
  pub room_id: i32,
  pub user_id: i32,
  pub draw_item_id: i32,
  // pub user: User,
  // pub draw_item: DrawItem,
  pub created_at: NaiveDateTime,
}
