// main.rs

mod error;
mod routes;

use clap::Parser;
use rocket::*;
use manager::{database, environment};

use crate::routes::health_check;
use crate::routes::members;
use crate::routes::capsuleers;
use crate::routes::skills;

#[derive(Parser)]
struct Cli {
    /// Name of the configuration
    #[arg(short, long, default_value = "local")]
    config: String,

    /// IP address on which the gateway is listening
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    /// Port on which the gateway is listening
    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    /// Identity Provider Key 
    #[arg(short, long, default_value = "precious")]
    secret: String,
}


#[launch]
async fn rocket() -> _ {
    let args = Cli::parse();
    let address: String = args.address.parse().expect("Invalid IP address");
    let port = args.port;
    let config = environment::EnvironmentManager::load_config(&args.config)
        .await
        .expect("Failed to load configuration");
    let db = database::DatabaseManager::revision(&config)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .configure(
            rocket::Config::figment()
            .merge(("address", address.clone()))
            .merge(("port", port.clone()))
        )
        .manage(db)
        .manage(config)
        .mount("/api/", routes![health_check])
        .mount("/api/members/", members::routes())
        .mount("/api/capsuleers/", capsuleers::routes())
        .mount("/api/skills/", skills::routes())
}