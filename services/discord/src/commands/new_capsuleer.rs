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

use crate::Handler;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    handle: &Handler
    ) -> Result<(), serenity::Error> {

    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = interaction.data.options().first()
    {
        let modal = CreateQuickModal::new("Capsuleer")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Capsuleer Name")
            .short_field("Basic Planetology")
            .short_field("Advanced Planetology")
            .short_field("Expert Planetology");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (capsuleer_name, basic, advanced, expert)
          = (&inputs[0], &inputs[1], &inputs[2], &inputs[3]);

        let db = &handle.db as &DatabaseConnection;
        if let Some(member) = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap() {
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
                } else {
                    let capsuleer = new_capsuleer(&db, capsuleer_name, member.id, handle.corporation.id)
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
                }
        } else {
            let member = new_member(&db, &user.tag(), handle.corporation.id).await.expect("Failed to add member to database");
            let capsuleer = new_capsuleer(&db, capsuleer_name, member.last_insert_id, handle.corporation.id)
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
        }
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!(
                        "**Register**: {capsuleer_name} to {0} with Planetology {1}{2}{3}",
                        user.tag(),
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
    CreateCommand::new("new_capsuleer").description("Register capsuleer with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "member", "The member to register a capsuleer for")
            .required(true),
    )
}