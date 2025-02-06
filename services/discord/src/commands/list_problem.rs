use prettytable::{Table, Row, Cell, format::FormatBuilder, row};
use serenity::all::CreateCommand;
use sea_orm::EntityTrait;

use manager::database::DatabaseConnection;
use manager::entities::prelude::Problem;

pub async fn run(
    db: &DatabaseConnection
    ) -> String {
    let problems = Problem::find()
        .all(db)
        .await
        .unwrap();
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .padding(1, 1)
            .build()
    );
    table.add_row(row!["ID", "Name", "Active"]);
    for problem in problems {
        table.add_row(Row::new(vec![
            Cell::new(&problem.id.to_string()),
            Cell::new(&problem.name),
            Cell::new(&problem.active.to_string()),
        ]));
    }
    format!("```\n{}\n```", table.to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list_problem").description("Print problems with eve-anchor")
}