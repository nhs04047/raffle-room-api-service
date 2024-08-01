use sea_orm::DatabaseConnection;

use crate::{
  domain::mapper::Mapper,
  repository::{
    draw as DrawRepository,
    draw_item as DrawItemRepository,
    joined_user as JoinedUserRepository
  },
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::structs;

fn alloc_draws(draw_items: Vec<structs::DrawItem>, users: Vec<structs::User>) -> Vec<structs::Draw> {

  let mut draws = Vec::new();

  for draw_item in &draw_items {
    for i in 1..draw_item.qty {
      let mut rng = thread_rng();
      if let Some(random_element) = users.choose(&mut rng) {
        println!("Selected: {}", random_element);

        let draw = Draw {
          id: draws.len() as i32 + 1,
          room_id: 1, // example room_id
          user_id: random_user.id,
          draw_item_id: draw_item.id,
          created_at: chrono::Utc::now().naive_utc(),
        };
       draws.push(draw);

        let index = users.iter().position(|&x| x == *random_element).unwrap();
        users.remove(index);
    }
    }
  }
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

