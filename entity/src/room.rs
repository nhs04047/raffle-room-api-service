

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "room")]
pub struct Room {
  #[sea_orm(primary_key, auto_increment = true)]
  pub id: i32,
  pub name: String,
  pub password: String,
  pub set_draw_include_owner: i32,
  pub set_draw_order: string,
  pub status: i32,
  pub created_at: DateTime,
  pub updated_at: DateTime
}