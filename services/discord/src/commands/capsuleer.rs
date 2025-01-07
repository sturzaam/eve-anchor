use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;
use serenity::all::CommandOptionType;
use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

use sea_orm::*;
use manager::database::DatabaseConnection;
use manager::database::sea_orm::EntityTrait;
use manager::entities::{prelude::*, *};

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
            .short_field("Capsuleer Name")
            .short_field("Basic Planetology")
            .short_field("Advanced Planetology")
            .short_field("Expert Planetology");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (capsuleer_name, basic, advanced, expert) = (&inputs[0], &inputs[1], &inputs[2], &inputs[3]);

        let db = db as &DatabaseConnection;
        let member = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap()
            .unwrap();

        let capsuleer = capsuleer::ActiveModel {
            name: ActiveValue::Set(capsuleer_name.to_string()),
            member_id: ActiveValue::Set(member.id),
            ..Default::default()
        };

        let saved_capsuleer = Capsuleer::insert(capsuleer.clone())
            .exec(db)
            .await
            .expect("Failed to add capsuleer to database");

        let skill = skill::ActiveModel {
            name: ActiveValue::Set("Planetology".to_owned()),
            basic: ActiveValue::Set(basic.parse::<i32>().unwrap_or(0)),
            advanced: ActiveValue::Set(advanced.parse::<i32>().unwrap_or(0)),
            expert: ActiveValue::Set(expert.parse::<i32>().unwrap_or(0)),
            capsuleer_id: ActiveValue::Set(saved_capsuleer.last_insert_id),
            ..Default::default()
        };

        let _ = Skill::insert(skill.clone())
            .exec(db)
            .await
            .expect("Failed to add skill to database");

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
        CreateCommandOption::new(CommandOptionType::User, "name", "The member to register")
            .required(true),
    )
}