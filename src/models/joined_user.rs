use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection,Set, EntityTrait, DbErr, ActiveModelTrait};
use entity::joined_user::{self, Entity as JoinedUser};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewUser {
  pub name: String,
  pub tag: String,
  pub room_id: i32,
  pub created_at: NaiveDateTime,
}


pub async fn get_joined_user_by_id (
  db: &DatabaseConnection,
  id: i32,
) -> Result<Option<joined_user::Model>, DbErr>{
  JoinedUser::find_by_id(id).one(db).await
}

pub async fn get_joined_users (
  db: &DatabaseConnection
) -> Result<Vec<joined_user::Model>, DbErr>{
  JoinedUser::find().all(db).await
}

pub async fn save_joined_user (
  db: &DatabaseConnection,
  data: NewUser
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