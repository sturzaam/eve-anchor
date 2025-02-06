use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;
use serenity::all::CommandOptionType;
use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

use material_lp::data::find_system;
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
        let modal = CreateQuickModal::new("Outpost")
            .timeout(std::time::Duration::from_secs(600))
            .short_field("Capsuleer")
            .short_field("Name")
            .short_field("System")
            .short_field("Planets")
            .short_field("Arrays");
        let response = interaction.quick_modal(ctx, modal).await?.unwrap();

        let inputs = response.inputs;
        let (capsuleer, name, system, planets, arrays)
            = (&inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]);

        let db = &handle.db as &DatabaseConnection;
        if find_system(system).is_none() {
            response
                .interaction.create_response(
                    ctx,
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                        format!("{system} is not found in Eve"),
                    )),
                )
                .await?;
                return Ok(());
        };
        if let Some(_member) = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap() {
            if let Some(capsuleer) = Capsuleer::find_by_name(capsuleer, &db)
                .await
                .unwrap() {
                    let _ = new_outpost(
                        &db,
                        name,
                        system,
                        planets.parse::<i32>().unwrap_or(0),
                        arrays.parse::<i32>().unwrap_or(0),
                        capsuleer.id,
                        None
                    )
                        .await
                        .expect("Failed to add outpost to database");
            } else {
                let member = new_member(&db, &user.tag(), handle.corporation.id).await.expect("Failed to add member to database");
                let capsuleer = new_capsuleer(&db, capsuleer, member.last_insert_id, handle.corporation.id)
                    .await
                    .expect("Failed to add capsuleer to database");
                let _ = new_outpost(
                    &db,
                    name,
                    system,
                    planets.parse::<i32>().unwrap_or(0),
                    arrays.parse::<i32>().unwrap_or(0),
                    capsuleer.last_insert_id,
                    None
                )
                    .await
                    .expect("Failed to add outpost to database");
            }
        } else {
            let member = new_member(&db, &user.tag(), handle.corporation.id).await.expect("Failed to add member to database");
            let capsuleer = new_capsuleer(&db, capsuleer, member.last_insert_id, handle.corporation.id)
                .await
                .expect("Failed to add capsuleer to database");
                let _ = new_outpost(
                    &db,
                    name,
                    system,
                    planets.parse::<i32>().unwrap_or(0),
                    arrays.parse::<i32>().unwrap_or(0),
                    capsuleer.last_insert_id,
                    None
                )
                    .await
                    .expect("Failed to add outpost to database");
        }
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!(
                        "**Register**: {name} to {0} in {system} with {1} arrays for each of {2} planets",
                        user.tag(),
                        arrays,
                        planets,
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
    CreateCommand::new("new_outpost").description("Register outpost with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "member", "The member the outpost belongs to")
            .required(true),
    )
}