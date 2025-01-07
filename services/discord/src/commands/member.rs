use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use sea_orm::*;
use manager::database::DatabaseConnection;
use manager::database::sea_orm::EntityTrait;
use manager::entities::{prelude::*, *};

pub async fn run(options: &[ResolvedOption<'_>], db: &DatabaseConnection) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = options.first()
    {
        let db = db as &DatabaseConnection;

        if let Some(member) = Member::find_by_name(&user.tag(), &db)
            .await
            .unwrap() {
                return format!("{} already registered with eve-anchor", member.name);
        }
        let member = member::ActiveModel {
            name: ActiveValue::Set(user.tag()),
            ..Default::default()
        };

        let _ = Member::insert(member).exec(db).await;
            
        format!("{:?} registered with eve-anchor", user.tag())
    } else {
        "Please provide a valid member".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("member").description("Register member with eve-anchor").add_option(
        CreateCommandOption::new(CommandOptionType::User, "name", "The member to register")
            .required(true),
    )
}