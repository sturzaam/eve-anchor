// tests/mod.rs

use rocket::*;
use rocket::local::asynchronous::Client;
use rocket::http::{ContentType, Status};
use sea_orm::{
    ConnectionTrait,
    Database,
    DatabaseConnection,
    DbBackend,
    DbErr,
    Statement,
};

use manager::database::SessionTrait;
use manager::database::MigratorTrait;
use manager::database::postgres::create_database;
use manager::database::postgres::database_exists;
use manager::environment;
use manager::environment::Configuration;
use manager::migrator::Migrator;

use api::routes::capsuleers;
use api::routes::health_check;
use api::routes::members;
use api::routes::skills;

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

async fn local_rocket(env: &str) -> rocket::Rocket<Build> {
    let config = environment::EnvironmentManager::load_config(&env)
        .await.expect("Failed to load configuration");
    let db = DatabaseManager::revision(&config)
        .await.expect("Failed to connect to database");
    rocket::build()
        .manage(config)
        .manage(db)
        .mount("/api/", routes![health_check])
        .mount("/api/", members::routes())
        .mount("/api/", capsuleers::routes())
        .mount("/api/", skills::routes())
}

pub async fn create_client(env: &str) -> Client {
    let rocket = local_rocket(&env).await;
    Client::tracked(rocket).await.expect("Failed to create client")
}

pub async fn create_request(client: &Client, url: &str, request: &str) -> Status {
    let response = client.put(url)
        .header(ContentType::JSON)
        .body(request)
        .dispatch()
        .await;
    
    response.status()
}


pub const TEST_MEMBER_NAME: &str = "Sturzaam";
pub const TEST_MEMBER_REQUEST: &'static str = r##"{
    "name": "Sturzaam"
}"##;

pub const TEST_CAPSULEER_NAME: &str = "Aroff";
pub const TEST_CAPSULEER_REQUEST: &'static str = r##"{
    "name": "Aroff",
    "member": "Sturzaam"
}"##;

pub const TEST_SKILL_NAME: &str = "Planetology";
pub const TEST_SKILL_REQUEST: &'static str = r##"{
    "name": "Planetology",
    "capsuleer": "Aroff",
    "basic": 5,
    "advanced": 5,
    "expert": 5
}"##;

pub mod routes;