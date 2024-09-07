pub use sea_orm_migration::prelude::*;
pub use sea_orm::{
    Database,
    DatabaseConnection,
    DbErr,
    Statement
};

use crate::environment::Configuration;
use super::SessionTrait;

pub struct Session;

#[async_trait::async_trait]
impl SessionTrait for Session {
    async fn open(config: &Configuration) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(&config.url).await?;
        persistent_database(&db, &config.database).await?;

        let url = format!("{}/{}", &config.url, &config.database);
        Database::connect(&url).await
    }
}

pub async fn persistent_database(db: &DatabaseConnection, database_name: &str) -> Result<(), DbErr> {
    if database_exists(db, database_name).await? {
        println!("Database Exists: {}", database_name);
        return Ok(());    
    } else {
        create_database(db, database_name).await
    }
}

pub async fn database_exists(db: &DatabaseConnection, database_name: &str) -> Result<bool, DbErr> {
    let result = db
        .query_one(Statement::from_string(
            db.get_database_backend(),
            format!(
                "SELECT 1 FROM pg_database WHERE datname = '{}';",
                database_name
            ),
        ))
        .await?;
    Ok(result.is_some())
}

pub async fn create_database(db: &DatabaseConnection, database_name: &str) -> Result<(), DbErr> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", database_name),
    ))
    .await?;
    Ok(())
}