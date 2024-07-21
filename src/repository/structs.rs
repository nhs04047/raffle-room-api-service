use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewRoomModel {
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: i32,
  pub set_draw_order: String,
  pub status: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

#[derive(Deserialize, Debug)]
pub struct UpdateRoomModel {
  pub id: i32,
  pub name: Option<String>,
  pub password: Option<String>,
  pub set_draw_include_owner: Option<i32>,
  pub set_draw_order: Option<String>,
  pub status: Option<i32>,
  pub updated_at: NaiveDateTime
}

#[derive(Deserialize, Debug)]
pub struct NewUserModel {
  pub name: String,
  pub tag: String,
  pub room_id: i32,
  pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct NewDrawModel {
  pub room_id: i32,
  pub user_id: i32,
  pub draw_item_id: i32,
  pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct NewDrawItemModel {
  pub name: String,
  pub seq: i32,
  pub room_id: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct DrawItemInDrawDetailModel {
  pub id: i32,
  pub name: String,
  pub seq: i32
}

#[derive(Deserialize, Debug)]
pub struct UserInDrawDetailModel {
  pub id: i32,
  pub name: String,
  pub tag: i32,
}

#[derive(Deserialize, Debug)]
pub struct DrawDetaliModel {
  pub id: i32,
  pub darw_item: DrawItemInDrawDetailModel,
  pub user: UserInDrawDetailModel,
  pub user_id: i32,
  pub draw_item_id: i32,
  pub created_at: NaiveDateTime,
}