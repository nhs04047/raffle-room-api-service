use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection,Set, EntityTrait, DbErr, ActiveModelTrait};
use entity::draw::{self, Entity as Draw};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewDraw {
  pub room_id: i32,
  pub user_id: i32,
  pub draw_item_id: i32,
  pub created_at: NaiveDateTime,
}


pub async fn get_draws (
  db: &DatabaseConnection
) -> Result<Vec<draw::Model>, DbErr> {
  Draw::find().all(db).await
}

pub async fn save_room (
  db: &DatabaseConnection,
  data: NewDraw
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