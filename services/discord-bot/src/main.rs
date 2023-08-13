mod lib;
mod bot;

use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::command::{CommandOptionType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use tracing::{error, info};

use bot::{Bot};
use material_lp::{Material};


#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(1127975317529690243);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| { command.name("help").description("Explination of what to expect.") })
                .create_application_command(|command| { 
                    command
                        .name("config")
                        .description("Configure constraints for the material type.")
                        .create_option(|option| {
                            option
                                .name("requirements")
                                .description("The minimum requirements exported from eve echoes.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("type")
                                .description("The type of requirements `ship`, `structure`, or `corporation`.")
                                .kind(CommandOptionType::String)
                                .required(false)
                        })
                })
                .create_application_command(|command| { 
                    command
                        .name("material")
                        .description("Output the configured material type.")
                        .create_option(|option| {
                            option
                                .name("type")
                                .description("The type of requirements `ship`, `structure`, or `corporation`.")
                                .kind(CommandOptionType::String)
                                .required(false)
                        })
                })
                .create_application_command(|command| { 
                    command
                        .name("constellations")
                        .description("Configure constraints for the placement of outposts.")
                        .create_option(|option| {
                            option
                                .name("requirements")
                                .description("The number of outposts per constellation as KEY=value.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| { 
                    command
                        .name("solve")
                        .description("Solve constrained placement of arrary's maximizing total value.")
                        .create_option(|option| {
                            option
                                .name("days")
                                .description("The number of days harvesting material.")
                                .kind(CommandOptionType::Number)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("constellation")
                                .description("The name of the constellation you are interested in.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("type")
                                .description("The type of requirements ship, structure, or corporation.")
                                .kind(CommandOptionType::String)
                                .required(false)
                        })
                        .create_option(|option| {
                            option
                                .name("requirements")
                                .description("The minimum requirements exported from Eve Echoes.")
                                .kind(CommandOptionType::String)
                                .required(false)
                        })
                })                    
        }).await.unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "help" => "
Welcome to `eve-anchor` Discord bot. 

- The minimum material requirements list should be configured with `/config` command.
  Simply paste the export list from the Eve Echoes industry to the `requirements` option.
  The `type` option can be used to configure a default requirements list.
- The material requirements list can be returned with the `/material` command.
  You are required to provide the `type` of list you wish to see: 
    `ship`, `structure`, `corporation`(defaulted to `corporation`)
- The configuration of outposts accross constellations should be configured with `/constellations` command.
  Use `KEY=value` pairs for example: 
  `FY6-NK=3 R2-BT6=3 E-ILCH=3`
  This represents 3 outposts anchored in constellations `FY6-NK`, `R2-BT6`, and `E-ILCH`.
- When ready run the linear program to maximize total value with `/solve` command.
  You are required to provide the number of `days` to harvest and the `constellation` you would like output.
  Optionally you may provide:
  - the `type` of material to maximize: `ship`, `structure`, `corporation`(defaulted to `corporation`)
  - the `requirements` from your own list exported from Eve Echoes.  

**Note**: depending on your choices and anchored outposts the response may timeout...
Try again after 30 seconds as the results are cached...".to_owned(),
                "config" => self.handle_config(command.clone()),
                "constellations" => self.handle_constellations(command.clone()),
                "material" => self.handle_material(command.clone()),
                "solve" => self.handle_solver(command.clone()),
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let a_bot_token = std::env::var("ABOT_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = std::env::var("APP_ID")
        .expect("Expected an Application Id in the environment")
        .parse()
        .expect("Application Id must be a valid u64");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let bot = Bot {
        materials: Arc::new(Mutex::new(Vec::new())),
        ship_materials: Arc::new(Mutex::new(Vec::new())),
        structure_materials: Arc::new(Mutex::new(Vec::new())),
        corporation_materials: Arc::new(Mutex::new(Vec::new())),
        constellations: Arc::new(Mutex::new(Vec::new())),
    };
    let mut client = Client::builder(&a_bot_token, intents)
        .event_handler(bot)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
