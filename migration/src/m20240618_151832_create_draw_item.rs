use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JoinedUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JoinedUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JoinedUser::Name).string().not_null())
                    .col(ColumnDef::new(JoinedUser::RoomId).integer().not_null())
                    .col(ColumnDef::new(JoinedUser::Qty).integer().not_null())
                    .col(ColumnDef::new(JoinedUser::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-joined_user-room_id")
                            .from(JoinedUser::Table, JoinedUser::RoomId)
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
            .drop_table(Table::drop().table(JoinedUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum JoinedUser {
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
