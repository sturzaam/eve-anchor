use sea_orm::ActiveValue;
use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;
use serenity::all::CommandOptionType;
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
    
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = interaction.data.options().first()
    {
        let modal = CreateQuickModal::new("Problem Member")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Problem Name");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();
        let inputs = response.inputs;
        let problem_name = &inputs[0];

        let db = db as &DatabaseConnection;
        let problem = Problem::find_by_name(problem_name, &db)
            .await
            .unwrap()
            .unwrap();
        let capsuleers: Vec<(member::Model, Option<capsuleer::Model>)> = Member::find_capsuleer_by_name(&user.tag(), &db)
            .await
            .unwrap();

        for capsuleer in capsuleers {
            let outposts = Outpost::find_by_capsuleer(capsuleer.1.unwrap().id, &db)
                .await
                .unwrap();
            for outpost in outposts {
                let mut active_outpost: outpost::ActiveModel = outpost.into();
                active_outpost.problem_id = ActiveValue::Set(Some(problem.id));
                active_outpost
                    .update(db)
                    .await
                    .expect("Failed to update outpost");
            }
        }
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!("**Added**: {0} to {problem_name}", user.tag()),
                )),
            )
            .await?;
        return Ok(());
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
    CreateCommand::new("add_member_outpost_to_problem").description("Add the member outposts to problem").add_option(
        CreateCommandOption::new(CommandOptionType::User, "member", "The member to add outposts to problem")
            .required(true),
    )
}