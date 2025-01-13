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
        let modal = CreateQuickModal::new("Member")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Alliance")
            .short_field("Corporation");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (alliance_name, corporation_name) = (&inputs[0], &inputs[1]);

        let db = db as &DatabaseConnection;

        if let Some(corporation) = Corporation::find_by_name(corporation_name, &db)
            .await
            .unwrap() {
                response
                    .interaction
                    .create_response(
                        ctx,
                        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                            format!("{} already registered with eve-anchor", corporation.name),
                        )),
                    )
                    .await?;
                return Ok(());
        } else if let Some(alliance) = Alliance::find_by_name(alliance_name, &db)
            .await
            .unwrap() {
                let _ = new_corporation(&db, corporation_name, alliance.id)
                    .await
                    .expect("Failed to add corporation to database");
        } else {
            let alliance = new_alliance(&db, alliance_name)
                .await
                .expect("Failed to add alliance to database");
            let _ = new_corporation(&db, corporation_name, alliance.last_insert_id)
                .await
                .expect("Failed to add corporation to database");
        }

        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!("{} registered with eve-anchor", corporation_name),
                )),
            )
            .await?;
        
        Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("corporation").description("Register corporation with eve-anchor")
}