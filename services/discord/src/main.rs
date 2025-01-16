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

use manager::database::DatabaseConnection;
use manager::database::DatabaseManager;

struct Handler {
    db: DatabaseConnection,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "corporation" => {
                    commands::corporation::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "member" => {
                    commands::member::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "capsuleer" => {
                    commands::capsuleer::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "outpost" => {
                    commands::outpost::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "problem" => {
                    commands::problem::run(&ctx, &command, &self.db).await.unwrap();
                    None
                }
                "problem_outpost" => {
                    commands::problem_outpost::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
                "solve_problem" => {
                    commands::solve_problem::run(&ctx, &command, &self.db).await.unwrap();
                    None
                },
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
                commands::corporation::register(),
                commands::member::register(),
                commands::capsuleer::register(),
                commands::outpost::register(),
                commands::problem::register(),
                commands::problem_outpost::register(),
                commands::solve_problem::register(),
            ])
            .await;
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
    let bot = Handler {
        db,
    };


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

