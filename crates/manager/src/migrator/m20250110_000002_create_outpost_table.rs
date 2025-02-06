use sea_orm_migration::prelude::*;

use super::m20240804_000002_create_capsuleer_table::Capsuleer;
use super::m20250110_000001_create_problem_table::Problem;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250110_000002_create_outpost_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Outpost::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Outpost::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Outpost::Name).string().not_null())
                    .col(ColumnDef::new(Outpost::System).string().not_null())
                    .col(ColumnDef::new(Outpost::Planets).integer().not_null())
                    .col(ColumnDef::new(Outpost::Arrays).integer().not_null())
                    .col(ColumnDef::new(Outpost::CapsuleerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-capsuler-outpost_id")
                            .from(Outpost::Table, Outpost::CapsuleerId)
                            .to(Capsuleer::Table, Capsuleer::Id),
                    )
                    .col(ColumnDef::new(Outpost::ProblemId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-outpost_id")
                            .from(Outpost::Table, Outpost::ProblemId)
                            .to(Problem::Table, Problem::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Outpost::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
pub enum Outpost {
    Table,
    Id,
    Name,
    System,
    Planets,
    Arrays,
    CapsuleerId,
    ProblemId
}
