use sea_orm_migration::prelude::*;

use super::m20250109_000002_create_corporation_table::Corporation;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250109_000004_alter_capsuleer_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Capsuleer::Table)
                    .add_column(ColumnDef::new(Capsuleer::CorporationId).integer().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-corporation-capsuleer_id")
                            .from_tbl(Capsuleer::Table)
                            .from_col(Capsuleer::CorporationId)
                            .to_tbl(Corporation::Table)
                            .to_col(Corporation::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Capsuleer::Table)
                    .drop_foreign_key(Alias::new("fk-corporation-capsuleer_id"))
                    .drop_column(Capsuleer::CorporationId)
                    .to_owned()
            )
            .await
    }
}



#[derive(DeriveIden)]
pub enum Capsuleer {
    Table,
    CorporationId,
}
