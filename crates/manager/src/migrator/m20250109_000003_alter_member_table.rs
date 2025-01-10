use sea_orm_migration::prelude::*;

use super::m20250109_000002_create_corporation_table::Corporation;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250109_000003_alter_member_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(Member::Table)
                .add_column(ColumnDef::new(Member::CorporationId).integer().not_null())
                .add_foreign_key(
                    TableForeignKey::new()
                        .name("fk-corporation-member_id")
                        .from_tbl(Member::Table)
                        .from_col(Member::CorporationId)
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
                    .table(Member::Table)
                    .drop_foreign_key(Alias::new("fk-corporation-member_id"))
                    .drop_column(Member::CorporationId)
                    .to_owned()
            )
            .await
    }
}


#[derive(DeriveIden)]
pub enum Member {
    Table,
    CorporationId,
}
