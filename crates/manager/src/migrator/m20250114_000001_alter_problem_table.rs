use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250114_000001_alter_problem_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Problem::Table)
                    .add_column(ColumnDef::new(Problem::Constraint).binary())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Problem::Table)
                    .drop_column(Problem::Constraint)
                    .to_owned()
            )
            .await
    }
}



#[derive(DeriveIden)]
pub enum Problem {
    Table,
    Constraint,
}
