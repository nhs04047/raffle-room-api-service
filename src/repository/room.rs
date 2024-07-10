use sea_orm::{DatabaseConnection,Set, EntityTrait, DbErr, ActiveModelTrait};
use entity::room::{self, Entity as Room};

use super::structs;

pub async fn get_room_by_id (
  db: &DatabaseConnection,
  id: i32,
) -> Result<Option<room::Model>, DbErr>{
  Room::find_by_id(id).one(db).await
}

pub async fn get_rooms (
  db: &DatabaseConnection
) -> Result<Vec<room::Model>, DbErr>{
  Room::find().all(db).await
}

pub async fn insert_room (
  db: &DatabaseConnection,
  data: structs::NewRoomModel
) -> Result<room::Model, DbErr> {
  let room_model = room::ActiveModel {
    name: Set(data.name.to_owned()),
    password: Set(data.password.to_owned()),
    set_draw_include_owner: Set(data.set_draw_include_owner),
    set_draw_order: Set(data.set_draw_order.to_owned()),
    status: Set(data.status),
    created_at: Set(data.created_at),
    updated_at: Set(data.updated_at),
    ..Default::default()
  };

room_model.insert(db).await
}

pub async fn update_room (
  db: &DatabaseConnection,
  data: structs::UpdateRoomModel
) -> Result<room::Model, DbErr> {
  let room_model = Room::find_by_id(data.id).one(db).await?;
  let mut room_model: room::ActiveModel = room_model.unwrap().into();

  if let Some(ref name) = data.name {
    room_model.name = Set(name.to_owned());
}

if let Some(ref password) = data.password {
  room_model.password = Set(password.to_owned());
}

if let Some(ref set_draw_include_owner) = data.set_draw_include_owner {
  room_model.set_draw_include_owner = Set(*set_draw_include_owner)
}

if let Some(ref set_draw_order) = data.set_draw_order {
  room_model.set_draw_order = Set(set_draw_order.to_owned())
}
if let Some(ref status) = data.status {
  room_model.status = Set(*status)
}


room_model.update(db).await
}