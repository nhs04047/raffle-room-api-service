use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Draw::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Draw::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Draw::RoomId).integer().not_null())
                    .col(ColumnDef::new(Draw::UserId).integer().not_null())
                    .col(ColumnDef::new(Draw::DrawItemId).integer().not_null())
                    .col(ColumnDef::new(Draw::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-draw-room_id")
                            .from(Draw::Table, Draw::RoomId)
                            .to(Room::Table, Room::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-draw-user_id")
                            .from(Draw::Table, Draw::UserId)
                            .to(JoinedUser::Table, JoinedUser::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-draw-draw_item_id")
                            .from(Draw::Table, Draw::DrawItemId)
                            .to(DrawItem::Table, DrawItem::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Draw::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Draw {
    Table,
    Id,
    RoomId,
    UserId,
    DrawItemId,
    CreatedAt,
}

#[derive(Iden)]
enum Room {
    Table,
    Id,
}

#[derive(Iden)]
enum JoinedUser {
    Table,
    Id,
}

#[derive(Iden)]
enum DrawItem {
    Table,
    Id,
}
