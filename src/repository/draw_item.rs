use sea_orm::{ActiveModelTrait, DatabaseConnection, ColumnTrait, DbErr, EntityTrait, InsertResult, QueryFilter, Set};
use entity::draw_item::{self, Entity as DrawItem};

use super::structs;

pub async fn get_draw_items (
  db: &DatabaseConnection
) -> Result<Vec<draw_item::Model>, DbErr> {
  DrawItem::find().all(db).await
}

pub async fn get_draws_items_by_room_id (
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<draw_item::Model>, DbErr> {
  DrawItem::find().filter(draw_item::Column::RoomId.eq(room_id)).all(db).await
}


pub async fn insert_draw_item (
  db: &DatabaseConnection,
  data: structs::NewDrawItemModel
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
  data: Vec<structs::NewDrawItemModel>
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