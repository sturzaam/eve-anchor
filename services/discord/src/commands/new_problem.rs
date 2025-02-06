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
        let modal = CreateQuickModal::new("Problem")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Problem Name")
            .paragraph_field("Material List Exported from Eve Echoes");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (problem_name, constraints) = (&inputs[0], &inputs[1]);
        let db = &handle.db as &DatabaseConnection;
        if let Some(member) = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap() {
                new_problem(
                    &db,
                    problem_name,
                    constraints.to_string().into(),
                    member.id,
                    handle.corporation.id,
                    Some(handle.alliance.id)
                ).await.expect("Failed to add problem to database");
        } else {
            let member = new_member(&db, &user.tag(), handle.corporation.id).await.expect("Failed to add member to database");
            new_problem(
                &db,
                problem_name,
                constraints.to_string().into(),
                member.last_insert_id,
                handle.corporation.id,
                Some(handle.alliance.id)
            ).await.expect("Failed to add problem to database");

        }
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!("**Problem**: {problem_name} created for {0} in {1}", &user.tag(), handle.corporation.name),
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
    CreateCommand::new("new_problem").description("Solve problem with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "member", "The member to solve a problem for")
            .required(true),
    )
}