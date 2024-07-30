use sea_orm::DatabaseConnection;

use crate::{
  domain::mapper::Mapper,
  repository::{
    draw as DrawRepository,
    draw_item as DrawItemRepository,
    joined_user as JoinedUserRepository
  },

};

use super::structs;

fn alloc_draws(darw_items: Vec<structs::DrawItem>, users: Vec<structs::User>) -> Vec<structs::Draw> {
  
}

pub async fn get_draws_by_room_id(
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<structs::Draw>, String> {
  let draw_models_result = DrawRepository::get_draws_by_room_id(db, room_id).await;
  let draw_models = draw_models_result.map_err(|e| format!("DrawService::get_draws_by_room_id: {}", e.to_string()))?;
  let draws: Vec<structs::Draw> = draw_models.iter().map(|model| structs::Draw::map(model)).collect();

  if draws.len() > 0 {
    return Ok(draws)
  }

  let draw_item_models_result = DrawItemRepository::get_draws_items_by_room_id(db, room_id).await;
  let draw_item_models = draw_item_models_result.map_err(|e| format!("DrawService::get_draws_items_by_room_id: {}", e.to_string()))?;
  let draw_items: Vec<structs::DrawItem> = draw_item_models.iter().map(|model| structs::DrawItem::map(model)).collect();

  let user_models_result = JoinedUserRepository::get_joined_users_by_room_id(db, room_id).await;
  let user_models = user_models_result.map_err(|e| format!("DrawService::get_joined_users_by_room_id: {}", e.to_string()))?;
  let users: Vec<structs::User> = user_models.iter().map(|model| structs::User::map(model)).collect();


  
}

