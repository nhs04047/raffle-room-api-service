use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "room")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = true)]
  pub id: i32,
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: i32,
  pub set_draw_order: String,
  pub status: i32,
  pub created_at: DateTime,
  pub updated_at: DateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::joined_user::Entity")]
  JoinedUser,
  #[sea_orm(has_many = "super::draw_item::Entity")]
  DrawItem,
  #[sea_orm(has_many = "super::draw::Entity")]
  Draw
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

impl Related<super::draw::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Draw.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}