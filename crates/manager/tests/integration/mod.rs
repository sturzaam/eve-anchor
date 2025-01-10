// tests/integration/mod.rs

use manager::environment::*;
use manager::migrator::Migrator;
use manager::database::SessionTrait;
use manager::database::postgres::{
    database_exists,
    create_database
};

pub use sea_orm_migration::prelude::*;
pub use sea_orm::{
    Database,
    DatabaseConnection,
    DbBackend,
    DbErr,
    Statement
};


pub struct Session;
pub struct DatabaseManager;

#[async_trait::async_trait]
impl SessionTrait for Session {
    async fn open(config: &Configuration) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(&config.url).await?;
        ephemeral_database(&db, &config.database).await?;

        let url = format!("{}/{}", &config.url, &config.database);
        Database::connect(&url).await
    }
}

pub async fn ephemeral_database(db: &DatabaseConnection, database_name: &str) -> Result<(), DbErr> {
    if database_exists(db, database_name).await? {
        drop_database(db, database_name).await?;
    }
    create_database(db, database_name).await
}

pub async fn drop_database(db: &DatabaseConnection, database_name: &str) -> Result<(), DbErr> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("DROP DATABASE \"{}\";", database_name),
    ))
    .await?;
    Ok(())
}

impl DatabaseManager {
    pub async fn revision(config: &Configuration) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(&config.url).await?;
        let db = match db.get_database_backend() {
            DbBackend::Postgres => Session::open(&config).await?,
            DbBackend::MySql => todo!(),
            DbBackend::Sqlite => todo!()
        };

        Migrator::refresh(&db).await?;

        Ok(db)
    }
}

pub const TEST_ALLIANCE_NAME: &str = "Alliance";
pub const TEST_CORPORATION_NAME: &str = "Corporation";
pub const TEST_CAPSULEER_NAME: &str = "Capsuleer";
pub const TEST_MEMBER_NAME: &str = "Member";
pub const TEST_SKILL_NAME: &str = "Planetology";

pub mod database;