use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "joined_user")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = true)]
  pub id: i32,
  pub room_id: i32,
  pub user_id: i32,
  pub draw_item_id: i32,
  pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "suber::room::Entity",
    from = "Column::RoomId",
    to = "super::room::Column::Id",
    on_update = "Cascade",
    on_delete = "SetNull"
  )]
  Room,
  #[sea_orm(
    belongs_to = "super::joined_user::Entity",
    from = "Column::UserId",
    to = "super::joined_user::Column::Id")]
  JoinedUser,
  #[sea_orm(
    belongs_to = "super::draw_item::Entity",
    from = "Column::DrawItemId",
    to = "super::draw_item::Column::Id")]
  DrawItem,
}

impl Related<super::room::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::Room.def()
  }
}

impl Related<super::joined_user::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::JoinedUser.def()
  }
}

impl Related<super::draw_item::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::DrawItem.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}