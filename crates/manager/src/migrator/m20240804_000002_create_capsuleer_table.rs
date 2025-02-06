use sea_orm_migration::prelude::*;

use super::m20240804_000001_create_member_table::Member;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240804_000002_create_capsuleer_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Capsuleer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Capsuleer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Capsuleer::Name).string().not_null())
                    .col(ColumnDef::new(Capsuleer::Active).boolean().not_null().default(true))
                    .col(ColumnDef::new(Capsuleer::MemberId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member-capsuleer_id")
                            .from(Capsuleer::Table, Capsuleer::MemberId)
                            .to(Member::Table, Member::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Capsuleer::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
pub enum Capsuleer {
    Table,
    Id,
    Name,
    Active,
    MemberId,
}
