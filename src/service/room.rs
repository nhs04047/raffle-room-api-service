use sea_orm::DatabaseConnection;

use super::structs;

pub async fn create_room(
  db: &DatabaseConnection,
  data: structs::NewRoom
) -> Result<structs::Room, Err> {
  
}