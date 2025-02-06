pub use sea_orm_migration::prelude::*;
pub use sea_orm::{
    Database,
    DatabaseConnection,
    DbBackend,
    DbErr,
};

pub mod postgres;
pub mod mysql;

use crate::migrator::Migrator;
use crate::environment::Configuration;

pub struct DatabaseManager;

impl DatabaseManager {
    pub async fn revision(config: &Configuration) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(&config.url).await?;
        let db = match db.get_database_backend() {
            DbBackend::MySql => mysql::Session::open(&config).await?,
            DbBackend::Postgres => postgres::Session::open(&config).await?,
            DbBackend::Sqlite => db,
        };

        Migrator::up(&db, None).await?;

        Ok(db)
    }
}

#[async_trait::async_trait]
pub trait SessionTrait {
    async fn open(config: &Configuration) -> Result<DatabaseConnection, DbErr>;
}