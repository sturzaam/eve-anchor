use crate::environment::{Configuration, EnvErr};
use super::EnvironmentTrait;

pub struct Environment;

#[async_trait::async_trait]
impl EnvironmentTrait for Environment {
    async fn load() -> Result<Configuration, EnvErr> {
        Ok(Configuration {
            url: "postgres://postgres:precious@localhost:5432".to_owned(),
            database: "eve-anchor-db".to_owned(),
        })
    }
}
