use sea_orm_migration::prelude::*;

use super::m20240804_000001_create_member_table::Member;
use super::m20250109_000002_create_corporation_table::Corporation;
use super::m20250109_000001_create_alliance_table::Alliance;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250110_000001_create_problem_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Problem::Table)
                    .col(
                        ColumnDef::new(Problem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Problem::Name).string().not_null())
                    .col(ColumnDef::new(Problem::Active).boolean().not_null().default(true))
                    .col(ColumnDef::new(Problem::MemberId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-member_id")
                            .from(Problem::Table, Problem::MemberId)
                            .to(Member::Table, Member::Id),
                    )
                    .col(ColumnDef::new(Problem::CorporationId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-corporation_id")
                            .from(Problem::Table, Problem::CorporationId)
                            .to(Corporation::Table, Corporation::Id),
                    )
                    .col(ColumnDef::new(Problem::AllianceId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-alliance_id")
                            .from(Problem::Table, Problem::AllianceId)
                            .to(Alliance::Table, Alliance::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Problem::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
pub enum Problem {
    Table,
    Id,
    Name,
    Active,
    MemberId,
    CorporationId,
    AllianceId,
}
