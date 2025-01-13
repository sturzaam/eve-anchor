use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

use manager::database::DatabaseConnection;
use manager::entities::prelude::*;
use manager::*;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    db: &DatabaseConnection
    ) -> Result<(), serenity::Error> {
    let modal = CreateQuickModal::new("Outpost")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Capsuleer")
        .short_field("Name")
        .short_field("System")
        .short_field("Planets")
        .short_field("Arrays");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (capsuleer, name, system, planets, arrays)
        = (&inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]);

    let db = db as &DatabaseConnection;
    
    if let Some(capsuleer) = Capsuleer::find_by_name(capsuleer, &db)
        .await
        .unwrap() {
            let _ = new_outpost(
                &db,
                name,
                system,
                planets.parse::<i32>().unwrap_or(0),
                arrays.parse::<i32>().unwrap_or(0),
                capsuleer.id,
                None
            )
                .await
                .expect("Failed to add outpost to database");
    } else {
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    "Please provide a valid capsuleer or run the /capsuleer command",
                )),
            )
            .await?;
        return Ok(());
    }

    response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                format!(
                    "**Register**: {name} in {system} with {0} arrays for each of {1} planets",
                    arrays,
                    planets,
                ),
            )),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("outpost").description("Register outpost with eve-anchor")
}