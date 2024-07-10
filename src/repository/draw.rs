use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,ColumnTrait, QueryFilter, Set};
use entity::draw::{self, Entity as Draw};

use super::structs;

pub async fn get_draws (
  db: &DatabaseConnection
) -> Result<Vec<draw::Model>, DbErr> {
  Draw::find().all(db).await
}

pub async fn get_draws_by_room_id (
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<draw::Model>, DbErr> {
  Draw::find().filter(draw::Column::RoomId.eq(room_id)).all(db).await
}

pub async fn insert_draw (
  db: &DatabaseConnection,
  data: structs::NewDrawModel
) -> Result<draw::Model, DbErr> {
  let draw_model = draw::ActiveModel {
    room_id: Set(data.room_id),
    user_id: Set(data.user_id),
    draw_item_id: Set(data.draw_item_id),
    created_at: Set(data.created_at),
    ..Default::default()
  };

  draw_model.insert(db).await
}