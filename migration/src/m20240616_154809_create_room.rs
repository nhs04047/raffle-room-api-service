use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::Name).string().not_null())
                    .col(ColumnDef::new(Room::Password).string().not_null())
                    .col(ColumnDef::new(Room::SetDrawIncludeOwner).integer().not_null())
                    .col(ColumnDef::new(Room::SetDrawOrder).string().not_null())
                    .col(ColumnDef::new(Room::Status).integer().not_null())
                    .col(ColumnDef::new(Room::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Room::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Room {
    Table,
    Id,
    Name,
    Password,
    SetDrawIncludeOwner,
    SetDrawOrder,
    Status,
    CreatedAt,
    UpdatedAt,
}
