use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250109_000001_create_alliance_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alliance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alliance::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alliance::Name).string().not_null())
                    .col(ColumnDef::new(Alliance::Active).boolean().not_null().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alliance::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
pub enum Alliance {
    Table,
    Id,
    Name,
    Active
}
