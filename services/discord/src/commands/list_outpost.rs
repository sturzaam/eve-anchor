use prettytable::{Table, Row, Cell, format::FormatBuilder, row};
use serenity::all::CreateCommand;
use sea_orm::EntityTrait;

use manager::database::DatabaseConnection;
use manager::entities::prelude::Outpost;

pub async fn run(
    db: &DatabaseConnection
    ) -> String {
    let mut problems = Outpost::find()
        .all(db)
        .await
        .unwrap();
    problems.sort_by(|a, b| a.system.cmp(&b.system));
    
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .padding(1, 1)
            .build()
    );
    table.add_row(row!["ID", "Name", "System", "Planets", "Arrays"]);
    for problem in problems {
        table.add_row(Row::new(vec![
            Cell::new(&problem.id.to_string()),
            Cell::new(&problem.name),
            Cell::new(&problem.system),
            Cell::new(&problem.planets.to_string()),
            Cell::new(&problem.arrays.to_string()),
        ]));
    }
    format!("```\n{}\n```", table.to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list_outpost").description("Print outpost with eve-anchor")
}