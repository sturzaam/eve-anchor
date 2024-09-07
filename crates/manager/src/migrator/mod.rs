pub use sea_orm_migration::prelude::*;

mod m20240804_000001_create_member_table;
mod m20240804_000002_create_capsuleer_table;
mod m20240804_000003_create_skill_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240804_000001_create_member_table::Migration),
            Box::new(m20240804_000002_create_capsuleer_table::Migration),
            Box::new(m20240804_000003_create_skill_table::Migration),
        ]
    }
}
