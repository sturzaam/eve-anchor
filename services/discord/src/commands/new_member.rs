use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;
use serenity::all::CommandOptionType;
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

    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = interaction.data.options().first()
    {
        let modal = CreateQuickModal::new("Member")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Alliance")
            .short_field("Corporation");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (alliance_name, corporation_name) = (&inputs[0], &inputs[1]);

        let db = db as &DatabaseConnection;

        if let Some(member) = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap() {
                interaction.create_response(
                    ctx,
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                        format!("{} already registered with eve-anchor", member.name),
                    )),
                )
                .await?;
        }
        
        if let Some(corporation) = Corporation::find_by_name(corporation_name, &db)
            .await
            .unwrap() {
                let _ = new_member(&db, &user.tag(), corporation.id)
                    .await
                    .expect("Failed to add member to database");
        } else if let Some(alliance) = Alliance::find_by_name(alliance_name, &db)
            .await
            .unwrap() {
                let corporation = new_corporation(&db, corporation_name, alliance.id)
                    .await
                    .expect("Failed to add corporation to database");
                let _ = new_member(&db, &user.tag(), corporation.last_insert_id)
                    .await
                    .expect("Failed to add member to database");
        } else {
            let alliance = new_alliance(&db, alliance_name)
                .await
                .expect("Failed to add alliance to database");
            let corporation = new_corporation(&db, corporation_name, alliance.last_insert_id)
                .await
                .expect("Failed to add corporation to database");
            let _ = new_member(&db, &user.tag(), corporation.last_insert_id)
                .await
                .expect("Failed to add member to database");
        }

        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!("{:?} registered with eve-anchor", &user.tag()),
                )),
            )
            .await?;
        } else {
            interaction.create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    "Please provide a valid member",
                )),
            )
            .await?;
        }
        Ok(())
    }

pub fn register() -> CreateCommand {
    CreateCommand::new("new_member").description("Register member with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "name", "The member to register")
            .required(true),
    )
}