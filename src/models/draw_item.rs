use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection,Set, EntityTrait, DbErr, ActiveModelTrait, InsertResult};
use entity::draw_item::{self, Entity as DrawItem};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewDrawItem {
  pub name: String,
  pub seq: i32,
  pub room_id: i32,
  pub qty: i32,
  pub created_at: NaiveDateTime,
}


pub async fn get_draw_items (
  db: &DatabaseConnection
) -> Result<Vec<draw_item::Model>, DbErr> {
  DrawItem::find().all(db).await
}

pub async fn insert_draw_item (
  db: &DatabaseConnection,
  data: NewDrawItem
) -> Result<draw_item::Model, DbErr> {
  let draw_item_model = draw_item::ActiveModel {
    name: Set(data.name.to_owned()),
    room_id: Set(data.room_id),
    seq: Set(data.seq),
    qty: Set(data.qty),
    created_at: Set(data.created_at),
    ..Default::default()
  };

  draw_item_model.insert(db).await
}

pub async fn insert_draw_items (
  db: &DatabaseConnection,
  data: Vec<NewDrawItem>
) -> Result<InsertResult<entity::draw_item::ActiveModel>, DbErr> {
  let draw_item_models: Vec<draw_item::ActiveModel>  = data.iter().map(|item| {draw_item::ActiveModel {
    name: Set(item.name.to_owned()),
    room_id: Set(item.room_id),
    seq: Set(item.seq),
    qty: Set(item.qty),
    created_at: Set(item.created_at),
    ..Default::default()
  }}).collect();

  DrawItem::insert_many(draw_item_models).exec(db).await

}