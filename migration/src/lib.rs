pub use sea_orm_migration::prelude::*;

mod m20240616_154809_create_room;
mod m20240618_151811_create_joined_user;
mod m20240618_151824_create_draw;
mod m20240618_151832_create_draw_item;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240616_154809_create_room::Migration),
            Box::new(m20240618_151811_create_joined_user::Migration),
            Box::new(m20240618_151824_create_draw::Migration),
            Box::new(m20240618_151832_create_draw_item::Migration),
        ]
    }
}
