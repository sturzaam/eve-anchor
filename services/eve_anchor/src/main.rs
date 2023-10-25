mod bot;

use std::sync::{Arc, Mutex};
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::command::{CommandOptionType};

use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use tracing::{info};

use bot::{Bot};



#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(866820253107093515);

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
                        .name("report")
                        .description("Output the report of type specified.")
                        .create_option(|option| {
                            option
                                .name("type")
                                .description("The type of report `ship`, `structure`, `corporation`, or `outpost`.")
                                .kind(CommandOptionType::String)
                                .required(false)
                        })
                })
                .create_application_command(|command| { 
                    command
                        .name("outpost")
                        .description("Participcate by adding your outpost")
                        .create_option(|option| {
                            option
                                .name("outpost_name")
                                .description("The name of your outpost.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("outpost_system")
                                .description("The system your outpost is anchored in.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("capsuleer_name")
                                .description("The name of the capsuleer who anchored.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("corporation_name")
                                .description("Your current corporation short.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("alliance_name")
                                .description("Your corporation's alliance short.")
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
                                .name("outpost_name")
                                .description("The name of the outpost you are interested in.")
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
                })                    
        }).await.unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "help" => "
Welcome to `eve-anchor` Discord bot.

- `/outpost`: To participate use this command.
  - `outpost_name`: Use this in the `/solve` command.
  - `system_name`: To find optimal solution.
  - `capsuleer_name`: 
  - `corporation_name`: 
  - `alliance_name`:
- `/solve`: Run the constrained optimization to maximize total value.
  - `days`: Number of `days` to harvest.
  - `outpost_name`: The outpost to show.
  - `type`: `material` (default), `ship`, `structure`, or `corporation`
- `/config`: The minimum material requirements.
  - `requirements`: Paste the exported list from the Eve Echoes industry.
  - `type`: `material` (default), `ship`, `structure`, or `corporation`
- `/reports`: The minimum `material` or participating `outpost`'s.
  - `type`: Same as `/config type` with addition of `outpost`.

**Note**: depending on your choices and anchored outposts the response may timeout...
Try again after 30 seconds as the results are cached...".to_owned(),
                "config" => self.handle_config(command.clone()),
                "outpost" => self.handle_outpost(command.clone()),
                "report" => self.handle_report(command.clone()),
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