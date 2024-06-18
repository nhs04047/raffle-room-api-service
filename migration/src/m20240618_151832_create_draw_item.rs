use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DrawItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DrawItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DrawItem::Name).string().not_null())
                    .col(ColumnDef::new(DrawItem::RoomId).integer().not_null())
                    .col(ColumnDef::new(DrawItem::Qty).integer().not_null())
                    .col(ColumnDef::new(DrawItem::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-draw_item-room_id")
                            .from(DrawItem::Table, DrawItem::RoomId)
                            .to(Room::Table, Room::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DrawItem::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DrawItem {
    Table,
    Id,
    Name,
    RoomId,
    Qty,
    CreatedAt,
}

#[derive(Iden)]
enum Room {
    Table,
    Id,
}
