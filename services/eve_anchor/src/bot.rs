use std::path::{PathBuf};
use std::collections::HashMap;
use anyhow::Error;

use std::sync::{Arc, Mutex};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use serenity::async_trait;
use tracing::{info};

use material_lp::{
    manager::{Manager, Outpost},
    resource::{Material, CelestialResource},
    data::{get_constellation},
    create_corporation,
    create_alliance,
    create_member,
    create_capsuleer,
    outposts_per_constellation,
    solve_for_constellation
};

use eve_anchor::{
    report::{solution_table, outpost_table, material_table},
    application_id, load_outposts, parse_requirements,
};
use crate::commands::build_supported_commands;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CacheKey(String, String, i64);

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ListCacheKey(String, String, String, String);

pub struct Bot {
    pub alliance: Arc<Mutex<String>>,
    pub corporation: Arc<Mutex<String>>,
    pub lists_cache: Arc<Mutex<HashMap<ListCacheKey, Vec<Material>>>>,
    pub cache: Arc<Mutex<HashMap<CacheKey, Vec<(CelestialResource, f64)>>>>,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            alliance: Arc::new(Mutex::new(String::new())),
            corporation: Arc::new(Mutex::new(String::new())),
            lists_cache: Arc::new(Mutex::new(HashMap::new())),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn handle_admin(&self, command: ApplicationCommandInteraction) -> String {
        let mut alliance_lock = self.alliance.lock().expect("Mutex lock poisoned");
        let mut corporation_lock = self.corporation.lock().expect("Mutex lock poisoned");

        if let Some(alliance_option) = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "alliance")
        {
            if let Some(alliance_value) = alliance_option.value.as_ref() {
                *alliance_lock = alliance_value.as_str().expect("Option value is not a string").to_string();
                println!("{}", alliance_lock);
            }
        }

        if let Some(corporation_option) = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "corporation")
        {
            if let Some(corporation_value) = corporation_option.value.as_ref() {
                *corporation_lock = corporation_value.as_str().expect("Option value is not a string").to_string();
                println!("{}", corporation_lock);
            }
        }
        "administration updated.".to_owned()
    }

    fn create_manager(&self, command: &ApplicationCommandInteraction) -> Manager {
        let alliance = self.alliance.lock().unwrap().clone();
        let corporation = self.corporation.lock().unwrap().clone();

        if let Some(member) = command.data.options.iter().find(|opt| opt.name == "member") {
            if let Some(member_value) = member.value.clone() {
                let member_name = member_value.to_string().trim_matches('"').to_string();
                println!("Manager: {}, {}, {}", &member_name, &corporation, &alliance);
                return Manager::Member(create_member(&member_name, &corporation, &alliance));
            } else {
                println!("Manager: {}, {}", &corporation, &alliance);
                return Manager::Corporation(create_corporation(&corporation, &alliance));
            }
        }
        println!("Manager: {}, {}", &corporation, &alliance);
        Manager::Corporation(create_corporation(&corporation, &alliance))
    }

    pub fn add_list(&self, command: ApplicationCommandInteraction) -> String {
        let alliance = self.alliance.lock().unwrap().clone();
        let corporation = self.corporation.lock().unwrap().clone();
        let list_name = Self::option_string(&command, "name");

        let mut member_name = String::new();
        if let Some(member) = command.data.options.iter().find(|opt| opt.name == "member") {
            if let Some(member_value) = member.value.clone() {
                member_name = member_value.to_string();
            }
        };

        let requirements = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "requirements")
            .cloned();
        let requirements_string = requirements.unwrap().value.unwrap().to_string();
        let mut materials: Vec<Material> = Vec::new();
        materials = parse_requirements(requirements_string.clone())
            .expect("Failed to parse requirements.");

        let list_cache_key = ListCacheKey(alliance, corporation, member_name, list_name);
        let mut cache = self.lists_cache.lock().unwrap();
        cache.insert(list_cache_key, materials.clone());

        return material_table(materials)
    }

    pub fn delete_list(&self, command: ApplicationCommandInteraction) -> String {
        let alliance = self.alliance.lock().unwrap().clone();
        let corporation = self.corporation.lock().unwrap().clone();
        let list_name = Self::option_string(&command, "name");
        let mut member_name = String::new();
        if let Some(member) = command.data.options.iter().find(|opt| opt.name == "member") {
            if let Some(member_value) = member.value.clone() {
                member_name = member_value.to_string();
            }
        };

        let list_cache_key = ListCacheKey(alliance, corporation, member_name, list_name.clone());
        let mut lists_cache = self.lists_cache.lock().unwrap();

        // Use match to handle the Option returned by remove
        match lists_cache.remove(&list_cache_key) {
            Some(materials) => {
                // Do something with the removed materials if needed
                format!("List '{}' deleted!", list_name)
            }
            None => format!("List '{}' not found!", list_name),
        }
    }

    pub fn add_outpost(&self, command: ApplicationCommandInteraction) -> String {
        let capsuleer_name = Self::option_string(&command, "capsuleer_name");
        let outpost_name = Self::option_string(&command, "outpost_name");
        let system_name = Self::option_string(&command, "system_name");
        let available_planets: i32 = Self::option_int(&command, "available_planets");
        let available_arrays: i32 = Self::option_int(&command, "available_arrays");
        let manager = self.create_manager(&command);
        println!("Manager Directory: {}", &manager.get_dir().display());
        let mut loaded_manager = &mut manager.load_data().unwrap();
        println!("Manager Directory: {}", &loaded_manager.get_dir().display());
        if let Err(err) = &loaded_manager.add_outpost(
            &outpost_name,
            &system_name,
            &capsuleer_name,
            available_planets,
            available_arrays,
        ) {
            return format!("Error adding outpost: {}", err);
        }
        format!("Outpost '{}' added successfully!", outpost_name)
    }

    pub fn delete_outpost(&self, command: ApplicationCommandInteraction) -> String {
        let outpost_name = Self::option_string(&command, "outpost_name");
        let manager = self.create_manager(&command);
        println!("Manager Directory: {}", &manager.get_dir().display());
        let mut loaded_manager = manager.load_data().unwrap();
        println!("Manager Directory: {}", &loaded_manager.get_dir().display());
        if let Err(err) = loaded_manager.delete_outpost(&outpost_name) {
            format!("Error deleting outpost: {}", err);
        }
        format!("Outpost '{}' deleted!", outpost_name)
    }

    pub fn handle_help(&self, command: ApplicationCommandInteraction) -> String {
        if let Some(details_option) = command.data.options.iter().find(|opt| opt.name == "details") {
            match details_option.value.as_ref().expect("Option has no value").as_str() {
                Some("admin") => "
The `/admin` command can be used to set `alliance` and `corporation` values, for example:
```
/admin alliance:BRRR corporation:HMKI
```".to_owned(),
                Some("add_outpost") => "
The `/add_outpost` command will add a new outpost to the solution.
For example adding two outpost in the same system:
```
/add_outpost system_name:Tanoo available_planets:1 available_arrays:11 outpost_name:A capsuleer_name: Aroff
```
followed by
```
/add_outpost system_name:Tanoo available_planets:1 available_arrays:11 outpost_name:B capsuleer_name: Boggels
```".to_owned(),
                Some("delete_outpost") => "
The `/delete_outpost` command will remove an existing outpost from the solution.
For example:
```
/delete_outpost outpost_name:A
```".to_owned(),
                Some("add_list") => "
The `/add_list` command will add a new list to the solution.  This is the most difficult command.  From the Eve Echoes you need to \"Export to Clipboard\" the list of materials and paste it into the command.
For example:
```
/add_list name:materials requirements:ID	Names	Quantity	Valuation 
1	Tritanium	1	1.169 
2	Pyerite	1	3.661 
3	Mexallon	1	17.045 
4	Isogen	1	33.593 
5	Nocxium	1	267.05 
6	Zydrine	1	550.186 
7	Megacyte	1	2111.375 
8	Morphite	1	7748.286 
9	Heavy Water	1	37.947 
10	Suspended Plasma	1	109.627 
11	Liquid Ozone	1	245.336 
12	Ionic Solutions	1	690.802 
13	Oxygen Isotopes	1	1781.262 
14	Plasmoids	1	4628.729 
15	Lustering Alloy	1	316.82 
16	Sheen Compound	1	327.999 
17	Gleaming Alloy	1	119.336 
18	Condensed Alloy	1	263.494 
19	Precious Alloy	1	601.181 
20	Motley Compound	1	564.725 
21	Fiber Composite	1	182.756 
22	Lucent Compound	1	473.109 
23	Opulent Compound	1	353.626 
24	Glossy Compound	1	307.594 
25	Crystal Compound	1	508.004 
26	Dark Compound	1	410.025 
27	Reactive Gas	1	312.319 
28	Noble Gas	1	367.227 
29	Base Metals	1	693.686 
30	Heavy Metals	1	507.731 
31	Noble Metals	1	301.161 
32	Reactive Metals	1	853.377 
33	Toxic Metals	1	1005.739 
34	Industrial Fibers	1	1006.453 
35	Supertensile Plastics	1	460.411 
36	Polyaramids	1	135.87 
37	Coolant	1	591.003 
38	Condensates	1	291.235 
39	Construction Blocks	1	449.645 
40	Nanites	1	1141.602 
41	Silicate Glass	1	1227.779 
42	Smartfab Units	1	661.962 
43	Precious Metals	1	40195.953 
44	Non-CS Crystals	1	66606.491 
45	Polytextiles	1	224.0 
```".to_owned(),
                Some("delete_list") => "
The `/delete_list` command will remove an existing list from the solution.
For example:
```
/delete_list name:material
```".to_owned(),
                Some("solve") => "
The `/solve` command will suggest which planets and arrays your outposts should harvest.
For example:
```
/solve days:7 list:material outpost:A
```".to_owned(),
                Some("report") => "
The `/report` command will show you which outposts and lists are managed by the solver.
For example:
```
/report type:outpost
```".to_owned(),
                _ => "Invalid value for 'details' option.".to_owned(),
            }
        } else {"
    Welcome to `eve-anchor` Discord bot!

Here are available commands:

- **admin**: Administer sever level settings.
- **add_outpost**: Allow outpost to be managed.
- **delete_outpost**: Remove outpost from manager.
- **add_list**: Store list of materials.
- **delete_list**: Remove list of materials.
- **solve**: Render planets and array solution.
- **report**: Render outpost or list as a pretty table.

For additional details try `/help details <command>`".to_owned()
        }
    }

    pub fn handle_report_admin(&self, command: ApplicationCommandInteraction) -> String {
        let alliance = self.alliance.lock().unwrap();
        let corporation = self.corporation.lock().unwrap();
        format!("The current context:\n    alliance: {}\n    corporation: {}", *alliance, *corporation)
    }

    pub fn handle_report_list(&self, command: ApplicationCommandInteraction) -> String {
        let alliance = self.alliance.lock().unwrap().clone();
        let corporation = self.corporation.lock().unwrap().clone();
        let list_name = Self::option_string(&command, "name");

        let mut member_name = String::new();
        if let Some(member) = command.data.options.iter().find(|opt| opt.name == "member") {
            if let Some(member_value) = member.value.clone() {
                member_name = member_value.to_string();
            }
        };

        let list_cache_key = ListCacheKey(alliance, corporation, member_name, list_name);
        if let Some(materials) = self.lists_cache.lock().unwrap().get(&list_cache_key) {
            return material_table(materials.clone());
        }

        "List not found.".to_string()
    }

    pub fn handle_report_outpost(&self, command: ApplicationCommandInteraction) -> String {
        let manager = self.create_manager(&command);
        println!("Manager Directory: {}", &manager.get_dir().display());
        let mut loaded_manager = manager.load_data().unwrap();
        println!("Manager Directory: {}", &loaded_manager.get_dir().display());
        return outpost_table(loaded_manager.get_outposts().to_vec())
    }
    
    pub fn handle_solver(&self, command: ApplicationCommandInteraction) -> String {
        let alliance = self.alliance.lock().unwrap().clone();
        let corporation = self.corporation.lock().unwrap().clone();
        let days: f64 = Self::option_float(&command, "days");
        let list_name = Self::option_string(&command, "list_name");
        let outpost_name = Self::option_string(&command, "outpost_name");
        let manager = self.create_manager(&command);
        let mut loaded_manager = manager.load_data().unwrap();
        let mut outposts = loaded_manager.get_outposts().to_vec();

        let mut member_name = String::new();
        if let Some(member) = command.data.options.iter().find(|opt| opt.name == "member") {
            if let Some(member_value) = member.value.clone() {
                member_name = member_value.to_string();
            }
        };

        let cache_key = CacheKey(list_name.clone(), member_name.clone(), days as i64);
    
        if let Some(cached_result) = self.cache.lock().unwrap().get(&cache_key) {
            if let Some(outpost) = outposts.iter().find(|&o| o.name == outpost_name) {
                let constellation_id = outpost.constellation_id;
                let constellation = get_constellation(constellation_id);
                let constellation_name = constellation.unwrap().en_name.to_string();
                let response = solution_table(constellation_name.to_string(), (*cached_result).clone());
                return format!(
                    "To maximize total value for {} meeting the {} material requirements within {} days harvest the following:\n{}",
                    outpost_name,
                    list_name,
                    days,
                    response,
                );
            } else {
                return "Outpost not found.".to_string();
            }
        }

        let list_cache_key = ListCacheKey(alliance.clone(), corporation.clone(), member_name, list_name.clone());
        let materials_binding = self.lists_cache.lock().unwrap();
        if let Some(materials) = materials_binding.get(&list_cache_key) {
            println!("Outposts: {:#?}", outposts.clone());
            println!("Materials: {:#?}",materials.to_vec());
            let values = match solve_for_constellation(outposts.clone(), materials.to_vec(), days) {
                Ok(result) => result,
                Err(err) => {
                    drop(alliance);
                    drop(corporation);
                    return format!("solve_for_constellation failed: {:?}", err);
                }
            };
            let mut cache = self.cache.lock().unwrap();
            cache.insert(cache_key, values.clone());

            if let Some(outpost) = outposts.iter().find(|&o| o.name == outpost_name) {
                let constellation_id = outpost.constellation_id;
                let constellation = get_constellation(constellation_id);
                let constellation_name = constellation.unwrap().en_name.to_string();
                let response = solution_table(constellation_name.to_string(), values);
                format!(
                    "To maximize total value for {} meeting the {} material requirements within {} days harvest the following:\n{}",
                    outpost_name,
                    list_name,
                    days,
                    response,
                )
            } else {
                "Outpost not found.".to_string()
            }
        } else {
            return "List not found.".to_string();
        }
    }
    
    fn option_string(interaction: &ApplicationCommandInteraction, option_name: &str) -> String {
        let option = interaction
            .data
            .options
            .iter()
            .find(|opt| opt.name == option_name)
            .expect(&format!("Option: {} not found.", option_name));
    
        option
            .value
            .as_ref()
            .expect("Option has no value")
            .as_str()
            .expect("Option value is not a string")
            .to_string()
    }

    fn option_int(interaction: &ApplicationCommandInteraction, option_name: &str) -> i32 {
        let option = interaction
            .data
            .options
            .iter()
            .find(|opt| opt.name == option_name)
            .expect("Option not found.");
    
        option
            .value
            .as_ref()
            .expect("Option has no value.")
            .as_i64()
            .expect("Option value is not a 32 bit integer.") as i32
    }
    
    fn option_float(interaction: &ApplicationCommandInteraction, option_name: &str) -> f64 {
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
            .as_f64()
            .expect("Option value is not a float")
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(866820253107093515);
        GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            build_supported_commands().iter().for_each(|(command_name, command_description, command_options)| {
                commands.create_application_command(|command| {
                    command.name(command_name).description(command_description);
                    command_options.iter().for_each(|(option_name, option_description, option_type, is_required)| {
                        command.create_option(|option| {
                            option
                                .name(option_name)
                                .description(option_description)
                                .kind(*option_type)
                                .required(*is_required)
                        });
                    });

                    command
                });
            });

            commands

        }).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "help" => self.handle_help(command.clone()),
                "admin" => self.handle_admin(command.clone()),
                "report_admin" => self.handle_report_admin(command.clone()),
                "add_outpost" => self.add_outpost(command.clone()),
                "delete_outpost" => self.delete_outpost(command.clone()),
                "report_outpost" => self.handle_report_outpost(command.clone()),
                "add_list" => self.add_list(command.clone()),
                "delete_list" => self.delete_list(command.clone()),
                "report_list" => self.handle_report_list(command.clone()),
                "solve" => self.handle_solver(command.clone()),
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}