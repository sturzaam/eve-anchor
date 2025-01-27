use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;
use serenity::all::CommandOptionType;
use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::prelude::*;
use sea_orm::DeleteResult;

use manager::database::DatabaseConnection;
use manager::entities::prelude::Outpost;

use crate::Handler;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    handle: &Handler
    ) -> Result<(), serenity::Error> {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(outpost_name), ..
    }) = interaction.data.options().first()
    {
        let db = &handle.db as &DatabaseConnection;
        let result: DeleteResult = Outpost::delete_by_name(outpost_name, &db)
            .await
            .expect("Failed to delete outpost");
        if result.rows_affected > 0 {
                interaction.create_response(
                    ctx,
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                        format!("**Deleted** to delete: {outpost_name}"),
                    )),
                )
                .await?;
        } else {
            interaction.create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!("**Failed** to delete: {outpost_name}"),
                )),
            )
            .await?;
        }
    } else {
        interaction.create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                "Please provide a valid outpost",
            )),
        )
        .await?;
    }
    Ok(())
}
    

pub fn register() -> CreateCommand {
    CreateCommand::new("delete_outpost").description("Delete outpost with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::String, "outpost", "The outpost to delete")
            .required(true),
    )
}