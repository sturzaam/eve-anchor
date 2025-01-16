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
        let modal = CreateQuickModal::new("Problem")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Corporation Name")
            .short_field("Problem Name")
            .paragraph_field("Material List Exported from Eve Echoes");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (corporation_name, problem_name, constraints)
          = (&inputs[0], &inputs[1], &inputs[2]);

        let db = db as &DatabaseConnection;
        let member = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap()
            .unwrap();

        if let Some(corporation) = Corporation::find_by_name(corporation_name, &db)
            .await
            .unwrap() {
                let _ = new_problem(&db, problem_name, constraints.to_string().into(), member.id, corporation.id, None)
                    .await
                    .expect("Failed to add problem to database");
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
                    format!("**Problem**: {problem_name} created for {0} in {corporation_name}", &user.tag()),
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
    CreateCommand::new("problem").description("Solve problem with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "member", "The member to solve a problem for")
            .required(true),
    )
}