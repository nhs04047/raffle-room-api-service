use sea_orm::DatabaseConnection;

use crate::{
  repository::room as RoomRepository
};

use super::structs;

pub async fn create_room(
  db: &DatabaseConnection,
  data: structs::NewRoom
) -> Result<structs::Room, String> {
  let result = RoomRepository::insert_room(db, structs::NewRoom::to_model(data)).await;

  match result {
    Ok(model) => Ok(structs::Room::from_model(model)),
    Err(e) => Err(format!("RoomService::create_room: {}", e.to_string())),
  }
}