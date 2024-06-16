use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "joined_user")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = true)]
  pub id: i32,
  pub name: String,
  pub room_id: i32,
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
}

impl Related<super::room::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::Room.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}