use sea_orm::DatabaseConnection;

use crate::{
  domain::mapper::Mapper, repository::draw as DrawRepository
};

use super::structs;

pub async fn get_draws_by_room_id(
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<structs::Draw>, String> {
  let result = DrawRepository::get_draws_by_room_id(db, room_id).await;

  match result {
    Ok(models) => {
      Ok(models.into_iter().map(|model| structs::Draw::map(model)).collect())
    },
    Err(e) => Err(format!("JoinedUserService::get_users_by_room_id: {}", e.to_string()))
  }
}
