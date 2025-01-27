// src/main.rs

mod commands;
mod report;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use material_lp::cache::Cache;
use material_lp::data;
use manager::database::DatabaseConnection;
use manager::database::DatabaseManager;
use manager::entities::prelude::Alliance;
use manager::entities::prelude::Corporation;
use manager::entities::alliance;
use manager::entities::corporation;
use manager::new_alliance;
use manager::new_corporation;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CacheKey(String, String, i64);

struct Handler {
    alliance: alliance::Model,
    corporation: corporation::Model,
    db: DatabaseConnection,
    cache: Cache,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "new_member" => {
                    commands::new_member::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "new_capsuleer" => {
                    commands::new_capsuleer::run(&ctx, &command, &self).await.unwrap();
                    None
                },
                "new_outpost" => {
                    commands::new_outpost::run(&ctx, &command, &self).await.unwrap();
                    None
                },
                "new_problem" => {
                    commands::new_problem::run(&ctx, &command, &self).await.unwrap();
                    None
                },
                "delete_outpost" => {
                    commands::delete_outpost::run(&ctx, &command, &self).await.unwrap();
                    None
                },
                "add_member_outpost_to_problem" => {
                    commands::add_member_outpost_to_problem::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "solve_problem" => {
                    commands::solve_problem::run(&ctx, &command, &self).await.unwrap();
                    None
                },
                "list_outpost" => Some(
                    commands::list_outpost::run(&self.db).await
                ),
                "list_problem" => Some(
                    commands::list_problem::run(&self.db).await
                ),
                "list_problem_material" => Some(
                    commands::list_problem_material::run(&command.data.options(), &self.db).await
                ),
                _ => Some("not implemented.".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            std::env::var("GUILD_ID")
                .expect("Expected GUILD_ID in the .env file")
                .parse()
                .expect("GUILD_ID must be a valid u64"),
        );

        let _ = guild_id
            .set_commands(&ctx.http, vec![
                commands::new_member::register(),
                commands::new_capsuleer::register(),
                commands::new_outpost::register(),
                commands::new_problem::register(),
                commands::solve_problem::register(),
                commands::list_outpost::register(),
                commands::list_problem::register(),
                commands::list_problem_material::register(),
                commands::add_member_outpost_to_problem::register(),
                commands::delete_outpost::register(),
            ])
            .await;
    }
}

async fn validate(alliance: &str, corporation: &str, db: &DatabaseConnection) {
    if let Some(_) = Corporation::find_by_name(corporation, db)
        .await
        .unwrap() {
            return
    } else if let Some(alliance) = Alliance::find_by_name(alliance, db)
        .await
        .unwrap() {
        let _ = new_corporation(&db, corporation, alliance.id)
            .await
            .expect("Failed to add corporation to database");
    } else {
        let alliance = new_alliance(&db, alliance)
            .await
            .expect("Failed to add alliance to database");
        let _ = new_corporation(&db, corporation, alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("ABOT_TOKEN")
        .expect("Expected ABOT_TOKEN in the .env file");
    let config = manager::environment::EnvironmentManager::load_config("local")
        .await
        .expect("Failed to load configuration");
    let db = DatabaseManager::revision(&config)
        .await
        .expect("Failed to connect to database");
    validate("NRFD", "REEF", &db).await;
    let bot = Handler {
        alliance: Alliance::find_by_name("NRFD", &db).await.unwrap().unwrap(),
        corporation: Corporation::find_by_name("REEF", &db).await.unwrap().unwrap(),
        db,
        cache: Cache::new(std::time::Duration::from_secs(300)),
    };

    // Load lazy_static data
    let _ = &*data::CELESTIALS;
    let _ = &*data::CONSTELLATIONS;
    let _ = &*data::ITEMS;
    let _ = &*data::SYSTEMS;
    let _ = &*data::PLANETS;
    

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(bot)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

