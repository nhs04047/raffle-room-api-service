use sea_orm::DatabaseConnection;

use crate::{
  domain::mapper::Mapper, repository::draw_item as DrawItemRepository
};
use super::structs;

pub async fn create_draw_items(
  db: &DatabaseConnection,
  data: Vec<structs::NewDrawItem>
) -> Result<i32, String> {
  let new_draw_item_models = data.into_iter().map(|entity| structs::NewDrawItem::map(entity)).collect();
  let result = DrawItemRepository::insert_draw_items(db, new_draw_item_models).await;

  match result {
    Ok(models) => {
      Ok(models.last_insert_id)
    },
    Err(e) => Err(format!("DrawItemService::create_draw_items: {}", e.to_string()))
      
  }
}

pub async fn get_darw_items_room_id(
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<structs::DrawItem>, String> {
  let result = DrawItemRepository::get_draws_items_by_room_id(db, room_id).await;

  match result {
    Ok(models) => {
      Ok(models.into_iter().map(|model| structs::DrawItem::map(&model)).collect())
    },
    Err(e) => Err(format!("DrawItemService::get_darw_items_room_id: {}", e.to_string()))
      
  }
}