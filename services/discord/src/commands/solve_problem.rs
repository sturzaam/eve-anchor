use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

use material_lp::solve_for_constellation;
use material_lp::objective::parse_decomposed_list;
use material_lp::data::get_constellation;
use material_lp::data::find_constellation_by_system;
use manager::database::DatabaseConnection;
use manager::entities::prelude::*;
use manager::entities::*;

use crate::report::solution_table;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    db: &DatabaseConnection
    ) -> Result<(), serenity::Error> {
    let modal = CreateQuickModal::new("Solve Problem")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Problem Name")
        .short_field("Outpost Name")
        .short_field("Number of Days");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (problem_name, outpost_name, days) = (&inputs[0], &inputs[1], &inputs[2]);

    let db = db as &DatabaseConnection;
    let outpost = Outpost::find_by_name(outpost_name, &db)
        .await
        .unwrap()
        .unwrap();
    let problem_outposts: Vec<(problem::Model, Option<outpost::Model>)> = Problem::find_outposts_by_name(problem_name, &db)
            .await
            .unwrap();

    let constraint = std::str::from_utf8(&problem_outposts[0].0.constraint)
        .expect("Failed to convert constraint to string");
    let materials = parse_decomposed_list(constraint)
        .expect("Failed to parse constraint");

    let outposts: Vec<outpost::Model> = problem_outposts.iter().filter_map(|(_, outpost)| outpost.clone()).collect();
    if let Ok(result) = solve_for_constellation(outposts, materials, days.parse::<f64>().unwrap()) {
        let constellation_id = find_constellation_by_system(&outpost.system)
            .expect("Failed to find constellation by system");
        let constellation = get_constellation(*constellation_id);
        let constellation_name = constellation.unwrap().en_name.to_string();
        let solution = solution_table(constellation_name.to_string(), result);
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    format!(
                        "To maximize total value for {} meeting the {} material requirements within {} days harvest the following:\n{}",
                        outpost_name,
                        problem_name,
                        days,
                        solution,
                    ),
                )),
            )
            .await?;
        return Ok(());
    } else if !problem_outposts.is_empty() {
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
}

pub fn register() -> CreateCommand {
    CreateCommand::new("solve_problem").description("Solve the problem with related outpost using eve-anchor")
}