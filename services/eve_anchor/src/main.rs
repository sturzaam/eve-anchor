mod bot;
mod commands;

use dotenv::dotenv;
use serenity::prelude::*;
use bot::{Bot};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let a_bot_token = std::env::var("ABOT_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = std::env::var("APP_ID")
        .expect("Expected an Application Id in the environment")
        .parse()
        .expect("Application Id must be a valid u64");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let bot = Bot::new();
    let mut client = Client::builder(&a_bot_token, intents)
        .event_handler(bot)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
