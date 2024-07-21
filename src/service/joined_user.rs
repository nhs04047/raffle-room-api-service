use sea_orm::DatabaseConnection;

use crate::{
  domain::mapper::Mapper, repository::joined_user as JoinedUserRepository
};

use super::structs;

pub async fn create_user(
  db: &DatabaseConnection,
  data: structs::NewUser
) -> Result<structs::User, String> {
  let new_user_model = structs::NewUser::map(data);
  let result = JoinedUserRepository::save_joined_user(db, new_user_model).await;

  match result {
    Ok(model) => Ok(structs::User::map(model)),
    Err(e) => Err(format!("JoinedUserService::create_user: {}", e.to_string())),
  }
}

// pub async fn get_users(
//   db: &DatabaseConnection
// ) -> Result<Vec<structs::User>, String> {
//   let result = JoinedUserRepository::get_joined_users(db).await;

//   match result {
//     Ok(models) => {
//       Ok(models.into_iter().map(|model| structs::User::map(model)).collect())
//     },
//     Err(e) => Err(format!("JoinedUserService::get_users: {}", e.to_string()))
      
//   }
// }

pub async fn get_users_by_room_id(
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<structs::User>, String> {
  let result = JoinedUserRepository::get_joined_users_by_room_id(db, room_id).await;

  match result {
    Ok(models) => {
      Ok(models.into_iter().map(|model| structs::User::map(model)).collect())
    },
    Err(e) => Err(format!("JoinedUserService::get_users_by_room_id: {}", e.to_string()))
  }
}

pub async fn get_user(
  db: &DatabaseConnection,
  user_id: i32
) -> Result<Option<structs::User>, String> {
  let result = JoinedUserRepository::get_joined_user_by_id(db, user_id).await;

  match result {
    Ok(Some(model)) => {
      Ok(Some(structs::User::map(model)))
    },
    Ok(None) => Ok(None),
    Err(e) => {
        Err(format!("JoinedUserService::get_user: {}", e.to_string()))
    } 
  }
}

