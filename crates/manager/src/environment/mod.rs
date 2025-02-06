pub mod production;
pub mod development;
pub mod local;
pub mod test;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub url: String,
    pub database: String,
}

#[derive(Debug)]
pub enum EnvErr {
    InvalidEnvironment,
}

pub struct EnvironmentManager;

impl EnvironmentManager {
    pub async fn load_config(environment: &str) -> Result<Configuration, EnvErr> {
        let environment: &str = &std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| environment.to_string());
        match environment {
            "prod" => production::Environment::load().await,
            "dev" => development::Environment::load().await,
            "local" => local::Environment::load().await,
            "test" => test::Environment::load().await,
            _ => Err(EnvErr::InvalidEnvironment),
        }
    }
}

#[async_trait::async_trait]
pub trait EnvironmentTrait {
    async fn load() -> Result<Configuration, EnvErr>;
}