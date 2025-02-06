use sea_orm_migration::prelude::*;

use super::m20250109_000001_create_alliance_table::Alliance;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250109_000002_create_corporation_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Corporation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Corporation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Corporation::Name).string().not_null())
                    .col(ColumnDef::new(Corporation::Active).boolean().not_null().default(true))
                    .col(ColumnDef::new(Corporation::AllianceId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-alliance-corporation_id")
                            .from(Corporation::Table, Corporation::AllianceId)
                            .to(Alliance::Table, Alliance::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Corporation::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
pub enum Corporation {
    Table,
    Id,
    Name,
    Active,
    AllianceId,
}
