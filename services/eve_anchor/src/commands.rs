
use serenity::model::application::command::{CommandOptionType};

type EveAnchorCommands = Vec<(&'static str, &'static str, EveAnchorOptions)>;
type EveAnchorOptions = Vec<EveAnchorOption>;
type EveAnchorOption = (&'static str, &'static str, CommandOptionType, bool);

pub const REQUIRED: bool = true;
pub const OPTIONAL: bool = false;


// use serde::{Deserialize, Serialize};
// use std::{fs::File, io::Read};
// use serde_yaml;
// use serde_with_expand_env::with_expand_envs;


// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct EveAnchorCommand {
//     pub name: String,
//     pub description: String,
//     pub command_options: Vec<EveAnchorOption>,
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct EveAnchorOption {
//     pub name: String,
//     pub description: String,
//     pub kind: String,
//     pub required: bool,
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct CommandConfig {
//     pub commands: Vec<EveAnchorCommand>,
// }

// pub fn load_config(path: &str) -> CommandConfig {
//     let mut file = File::open(path).unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     serde_yaml::from_str(&contents).unwrap()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;

//     #[test]
//     fn test_load_commands() {
//         let path = "commands.yml";
//         let config = load_config(path);

//         assert_eq!(
//             config.commands[0].name,
//             "help"
//         );
//     }
// }

pub const outpost_name_option: EveAnchorOption = (
    "outpost_name",
    "Name of the outpost",
    CommandOptionType::String,
    REQUIRED
);

pub const type_option: EveAnchorOption = (
    "type",
    "The type of requirements `outpost`, `ship`, `structure`, or `corporation`.",
    CommandOptionType::String,
    OPTIONAL
);

pub const alliance_option: EveAnchorOption = (
    "alliance",
    "Name of the alliance",
    CommandOptionType::String,
    OPTIONAL
);

pub const corporation_option: EveAnchorOption = (
    "corporation",
    "Name of the corporation",
    CommandOptionType::String,
    OPTIONAL
);

pub const member_option: EveAnchorOption = (
    "member",
    "Name of the member",
    CommandOptionType::String,
    OPTIONAL
);

fn build_admin_options() -> EveAnchorOptions {
    let mut options = vec![
        alliance_option,
        corporation_option
    ];

    options
}

fn build_add_outpost_options() -> EveAnchorOptions {
    let mut options = vec![
        outpost_name_option,
        ("system_name", "Name of the system", CommandOptionType::String, REQUIRED),
        ("capsuleer_name", "Name of the system", CommandOptionType::String, REQUIRED),
        ("available_planets", "Available planets for solution", CommandOptionType::Integer, REQUIRED),
        ("available_arrays", "Available arrays for planet", CommandOptionType::Integer, REQUIRED),
        member_option,
    ];

    options
}

fn build_delete_outpost_options() -> EveAnchorOptions {
    let mut options = vec![
        outpost_name_option,
        member_option,
    ];

    options
}

fn build_add_list_options() -> EveAnchorOptions {
    let mut options = vec![
        ("name", "The name of the list.", CommandOptionType::String, REQUIRED),
        ("requirements", "List of minimum requirements exported from Eve Echoes.", CommandOptionType::String, REQUIRED),
        member_option,
    ];

    options
}

fn build_delete_list_options() -> EveAnchorOptions {
    let mut options = vec![
        ("name", "The name of the list.", CommandOptionType::String, REQUIRED),
        member_option,
    ];

    options
}

fn build_solve_options() -> EveAnchorOptions {
    let mut options = vec![
        ("days", "Number of days between hauling and fueling.", CommandOptionType::Number, REQUIRED),
        ("list_name", "The name of the list.", CommandOptionType::String, REQUIRED),
        ("outpost_name", "The name of the outpost.", CommandOptionType::String, REQUIRED),
        member_option,
    ];

    options
}

fn build_report_list_options() -> EveAnchorOptions {
    let mut options = vec![
        ("name", "The name of the list.", CommandOptionType::String, REQUIRED),
    ];

    options
}

fn build_report_outpost_options() -> EveAnchorOptions {
    let mut options = vec![
        member_option
    ];

    options
}

fn build_help_options() -> EveAnchorOptions {
    vec![
        ("details", "Name of the command.", CommandOptionType::String, OPTIONAL),
    ]
}

pub fn build_supported_commands() -> EveAnchorCommands {
    vec![
        ("help", "Describe available commands.", build_help_options()),
        ("admin", "Administer sever level settings.", build_admin_options()),
        ("report_admin", "Render admin settings.",vec![]),
        ("add_outpost", "Allow outpost to be managed.", build_add_outpost_options()),
        ("delete_outpost", "Remove outpost from manager.", build_delete_outpost_options()),
        ("report_outpost", "Render outpost as a pretty table.", build_report_outpost_options()),
        ("add_list", "Store list of materials.", build_add_list_options()),
        ("delete_list", "Remove list of materials.", build_delete_list_options()),
        ("report_list", "Render list as a pretty table.", build_report_list_options()),
        ("solve", "Maximize value of ISK meeting the list of materials.", build_solve_options()),
    ]
}