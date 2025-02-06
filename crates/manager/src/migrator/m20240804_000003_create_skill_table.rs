use sea_orm_migration::prelude::*;

use super::m20240804_000002_create_capsuleer_table::Capsuleer;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240804_000003_create_skill_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Skill::Table)
                    .col(
                        ColumnDef::new(Skill::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Skill::Name).string().not_null())
                    .col(ColumnDef::new(Skill::Basic).integer().not_null())
                    .col(ColumnDef::new(Skill::Advanced).integer().not_null())
                    .col(ColumnDef::new(Skill::Expert).integer().not_null())
                    .col(ColumnDef::new(Skill::CapsuleerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-capsuler-skill_id")
                            .from(Skill::Table, Skill::CapsuleerId)
                            .to(Capsuleer::Table, Capsuleer::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Skill::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
pub enum Skill {
    Table,
    Id,
    Name,
    Basic,
    Advanced,
    Expert,
    CapsuleerId,
}
