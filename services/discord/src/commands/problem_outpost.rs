use sea_orm::ActiveValue;
use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

use manager::database::sea_orm::ActiveModelTrait;
use manager::database::DatabaseConnection;
use manager::entities::prelude::*;
use manager::entities::*;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    db: &DatabaseConnection
    ) -> Result<(), serenity::Error> {
    let modal = CreateQuickModal::new("Problem Outpost")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Problem Name")
        .short_field("Outpost Name");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (problem_name, outpost_name)
        = (&inputs[0], &inputs[1]);

    let db = db as &DatabaseConnection;
    let problem = Problem::find_by_name(problem_name, &db).await.unwrap();
    let outpost = Outpost::find_by_name(outpost_name, &db).await.unwrap();

    if let (Some(problem), Some(outpost)) = (problem.clone(), outpost) {
            let mut active_outpost: outpost::ActiveModel = outpost.into();
            active_outpost.problem_id = ActiveValue::Set(Some(problem.id));
            active_outpost
                .update(db)
                .await
                .expect("Failed to update outpost");
    } else if let Some(_) = problem {
            response
                .interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                        "Please provide a valid outpost or run the /outpost command",
                    )),
                )
                .await?;
            return Ok(());
    } else {
        response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                "Please provide a valid problem or run the /problem command",
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
                format!("**Added**: {outpost_name} to {problem_name}"),
            )),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("problem_outpost").description("Add outpost to problem with eve-anchor")
}