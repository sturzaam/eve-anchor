use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use material_lp::create_outpost;

pub struct OutpostCommand {
    pub execute: Box<dyn Fn(&str) -> String>,
}

impl OutpostCommand {
    pub fn execute(&self, input: &str) -> String {
        (self.execute)(input)
    }
}

pub fn handle_outpost(command: ApplicationCommandInteraction) -> String {
    let outpost_name = option_string(&command, "outpost_name");
    let outpost_system = option_string(&command, "outpost_system");
    let capsuleer_name = option_string(&command, "capsuleer_name");
    let corporation_name = option_string(&command, "corporation_name");
    let alliance_name = option_string(&command, "alliance_name");
    let key = option_string(&command, "key");

    let outpost = create_outpost(
        &outpost_name,
        &outpost_system,
        &capsuleer_name,
        &corporation_name,
        &alliance_name,
        &key,
    );

    format!("Outpost: {}", outpost.name).to_owned()
}

fn option_string(interaction: &ApplicationCommandInteraction, option_name: &str) -> String {
    let option = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == option_name)
        .expect("Option not found");

    option
        .value
        .as_ref()
        .expect("Option has no value")
        .as_str()
        .expect("Option value is not a string")
        .to_string()
}