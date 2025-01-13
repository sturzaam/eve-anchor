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
        let modal = CreateQuickModal::new("Capsuleer")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Corporation Name")
            .short_field("Capsuleer Name")
            .short_field("Basic Planetology")
            .short_field("Advanced Planetology")
            .short_field("Expert Planetology");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (corporation_name, capsuleer_name, basic, advanced, expert)
          = (&inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]);

        let db = db as &DatabaseConnection;
        let member = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap()
            .unwrap();

        if let Some(capsuleer) = Capsuleer::find_by_name(capsuleer_name, &db)
            .await
            .unwrap() {
                response
                    .interaction
                    .create_response(
                        ctx,
                        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                            format!("{} already registered with eve-anchor", capsuleer.name),
                        )),
                    )
                    .await?;
                return Ok(());
        } else if let Some(corporation) = Corporation::find_by_name(corporation_name, &db)
            .await
            .unwrap() {
                let capsuleer = new_capsuleer(&db, capsuleer_name, member.id, corporation.id)
                    .await
                    .expect("Failed to add capsuleer to database");
                let _ = new_skill(
                    &db, 
                    "Planetology",
                    basic.parse::<i32>().unwrap_or(0),
                    advanced.parse::<i32>().unwrap_or(0),
                    expert.parse::<i32>().unwrap_or(0),
                    capsuleer.last_insert_id
                )
                .await
                .expect("Failed to add skill to database");
        } else {
            response
                .interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                        "Please provide a valid corporation or run the /corporation command",
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
                        "**Register**: {capsuleer_name} to {0} with Planetology {1}{2}{3}",
                        member.name,
                        basic,
                        advanced,
                        expert
                    ),
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
    CreateCommand::new("capsuleer").description("Register capsuleer with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "member", "The member to register a capsuleer for")
            .required(true),
    )
}