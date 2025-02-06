use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;
use serenity::all::CommandOptionType;
use serenity::all::CreateCommand;
use serenity::all::CreateCommandOption;

use material_lp::objective::parse_decomposed_list;
use manager::database::DatabaseConnection;
use manager::entities::prelude::Problem;


use crate::report::material_table;

pub async fn run(
    options: &[ResolvedOption<'_>],
    db: &DatabaseConnection
    ) -> String {
        let db = db as &DatabaseConnection;

        if let Some(ResolvedOption {
            value: ResolvedValue::String(name), ..
        }) = options.first()
        {
            if let Some(problem) = Problem::find_by_name(
                name,
                &db
            ).await.unwrap() {
                let constraint_str = std::str::from_utf8(&problem.constraint)
                    .expect("Failed to convert constraint to string");
                let materials = parse_decomposed_list(constraint_str)
                    .expect("Failed to parse constraint");
                return material_table(materials);
            } else {
                "Please provide a valid problem or run the /problem command".to_string()
            }
        } else {
            "Please provide a valid problem or run the /problem command".to_string()
        }
    }

pub fn register() -> CreateCommand {
    CreateCommand::new("list_problem_material").description("Print problem materials with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::String, "problem", "The problem name to list materials for.")
            .required(true),
    )
}