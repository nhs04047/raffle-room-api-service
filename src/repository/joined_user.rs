use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,ColumnTrait, Set};
use entity::joined_user::{self, Entity as JoinedUser};

use super::structs;

pub async fn get_joined_user_by_id (
  db: &DatabaseConnection,
  id: i32,
) -> Result<Option<joined_user::Model>, DbErr>{
  JoinedUser::find_by_id(id).one(db).await
}

pub async fn get_joined_users_by_room_id (
  db: &DatabaseConnection,
  room_id: i32
) -> Result<Vec<joined_user::Model>, DbErr> {
  JoinedUser::find().filter(joined_user::Column::RoomId.eq(room_id)).all(db).await
}

// pub async fn get_joined_users (
//   db: &DatabaseConnection
// ) -> Result<Vec<joined_user::Model>, DbErr>{
//   JoinedUser::find().all(db).await
// }

pub async fn save_joined_user (
  db: &DatabaseConnection,
  data: structs::NewUserModel
) -> Result<joined_user::Model, DbErr> {
  let joined_user = joined_user::ActiveModel {
    name: Set(data.name.to_owned()),
    tag: Set(data.tag.to_owned()),
    room_id: Set(data.room_id),
    created_at: Set(data.created_at),
    ..Default::default()
  };

  joined_user.insert(db).await
}