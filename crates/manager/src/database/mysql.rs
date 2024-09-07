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
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE IF NOT EXISTS `{}`;", &config.database),
        ))
        .await?;

        let url = format!("{}/{}", &config.url, &config.database);
        Database::connect(&url).await
    }
}
