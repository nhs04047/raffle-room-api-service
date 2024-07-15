use sea_orm::DatabaseConnection;

use crate::{
  domain::mapper::Mapper, repository::room as RoomRepository
};

use super::structs;

pub async fn create_room(
  db: &DatabaseConnection,
  data: structs::NewRoom
) -> Result<structs::Room, String> {
  let new_room_model = structs::NewRoom::map(data);
  let result = RoomRepository::insert_room(db, new_room_model).await;

  match result {
    Ok(model) => Ok(structs::Room::map(model)),
    Err(e) => Err(format!("RoomService::create_room: {}", e.to_string())),
  }
}

pub async fn get_rooms(
  db: &DatabaseConnection
) -> Result<Vec<structs::Room>, String> {
  let result = RoomRepository::get_rooms(db).await;

  match result {
    Ok(models) => {
      Ok(models.into_iter().map(|model| structs::Room::map(model)).collect())
    },
    Err(e) => Err(format!("RoomService::create_room: {}", e.to_string()))
      
  }
}

pub async fn get_room(
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Option<structs::Room>, String> {
  let result = RoomRepository::get_room_by_id(db, room_id).await;

  match result {
    Ok(Some(model)) => {
      Ok(Some(structs::Room::map(model)))
    },
    Ok(None) => Ok(None),
    Err(e) => {
        Err(format!("RoomService::get_room: {}", e.to_string()))
    } 
  }
}

pub async fn update_room(
  db: &DatabaseConnection,
  data: structs::UpdateRoom
) -> Result<structs::Room, String> {
  let update_room_model = structs::UpdateRoom::map(data);
  let result = RoomRepository::update_room(db, update_room_model).await;

  match result {
    Ok(model) => Ok(structs::Room::map(model)),
    Err(e) => Err(format!("RoomService::create_room: {}", e.to_string())),
  }
}