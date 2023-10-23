use std::path::{PathBuf};

use std::sync::{Arc, Mutex};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use eve_anchor::{
    application_id, load_outposts, parse_requirements,
    report::{solution_table, outpost_table, material_table}
};
use material_lp::{create_outpost, solve};
use material_lp::resource::{Material};

pub struct Bot {
    pub materials: Arc<Mutex<Vec<Material>>>,
    pub ship_materials: Arc<Mutex<Vec<Material>>>,
    pub structure_materials: Arc<Mutex<Vec<Material>>>,
    pub corporation_materials: Arc<Mutex<Vec<Material>>>,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            materials: Arc::new(Mutex::new(Vec::new())),
            ship_materials: Arc::new(Mutex::new(Vec::new())),
            structure_materials: Arc::new(Mutex::new(Vec::new())),
            corporation_materials: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_material(&self, material: Material) {
        let mut materials = self.materials.lock().unwrap();
        materials.push(material);
    }

    pub fn clear_materials(&self) {
        let mut materials = self.materials.lock().unwrap();
        materials.clear();
    }

    pub fn add_ship_material(&self, material: Material) {
        let mut ship_materials = self.ship_materials.lock().unwrap();
        ship_materials.push(material);
    }

    pub fn clear_ship_materials(&self) {
        let mut ship_materials = self.ship_materials.lock().unwrap();
        ship_materials.clear();
    }

    pub fn add_structure_material(&self, material: Material) {
        let mut structure_materials = self.structure_materials.lock().unwrap();
        structure_materials.push(material);
    }

    pub fn clear_structure_materials(&self) {
        let mut structure_materials = self.structure_materials.lock().unwrap();
        structure_materials.clear();
    }

    pub fn add_corporation_material(&self, material: Material) {
        let mut corporation_materials = self.corporation_materials.lock().unwrap();
        corporation_materials.push(material);
    }

    pub fn clear_corporation_materials(&self) {
        let mut corporation_materials = self.corporation_materials.lock().unwrap();
        corporation_materials.clear();
    }

    pub fn handle_config(&self, command: ApplicationCommandInteraction) -> String {
        let requirements = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "requirements")
            .cloned();
        let type_option = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "type")
            .cloned();
        let dirty_type = match type_option {
            Some(m) => m.value.unwrap().to_string(),
            None => "material".to_string(),
        };
        let clean_type = dirty_type.trim_matches('\"');
        let requirements_string = requirements.unwrap().value.unwrap().to_string();
        let mut materials: Vec<Material> = Vec::new();
        materials = parse_requirements(requirements_string.clone())
            .expect("Failed to parse requirements.");

        match clean_type {
            "ship" => {
                self.clear_ship_materials();
                materials
                    .iter()
                    .for_each(|material| self.add_ship_material(material.clone()));
                format!("{} Ship materials configured",
                    self.ship_materials.lock().unwrap().clone().len(),
                )
            },
            "structure" => {
                self.clear_structure_materials();
                materials
                    .iter()
                    .for_each(|material| self.add_structure_material(material.clone()));
                format!("{} Structure materials configured",
                    self.structure_materials.lock().unwrap().clone().len(),
                )
            },
            "corporation" => {
                self.clear_corporation_materials();
                materials
                    .iter()
                    .for_each(|material| self.add_corporation_material(material.clone()));
                format!("{} Corporation materials configured",
                    self.corporation_materials.lock().unwrap().clone().len(),
            )
            },
            "material" => {
                self.clear_materials();
                materials
                    .iter()
                    .for_each(|material| self.add_material(material.clone()));
                format!("{} Materials configured",
                    self.materials.lock().unwrap().clone().len(),
        )
            },
            _ => {
                "Invalid material type".to_owned()
            }
        }
    }

    pub fn handle_report(&self, command: ApplicationCommandInteraction) -> String {
        let type_option = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "type")
            .cloned();
        let dirty_type = match type_option {
            Some(m) => m.value.unwrap().to_string(),
            None => "material".to_string(),
        };
        let clean_type = dirty_type.trim_matches('\"');
        let _materials: Vec<Material> = Vec::new();
        match clean_type {
            "ship" => {
                material_table(self.ship_materials.lock().unwrap().clone())
            },
            "structure" => {   
                material_table(self.structure_materials.lock().unwrap().clone())
            },
            "corporation" => {
                material_table(self.corporation_materials.lock().unwrap().clone())
            },
            "outpost" => {
                let file_name = PathBuf::from(format!("./target/outpost/{}.bin", application_id()));
                outpost_table(load_outposts(&file_name).unwrap())
            },
            _ => {
                material_table(self.materials.lock().unwrap().clone())
            }
        }
    }
    
    pub fn handle_solver(&self, command: ApplicationCommandInteraction) -> String {
        let days: f64 = Self::option_float(&command, "days");
        let outpost_name = Self::option_string(&command, "outpost_name");
        let type_option = Self::option_string(&command, "type");
        let materials = Self::option_type(&self, &command, &type_option);
        let moved_materials = materials.clone();
        let file_name = PathBuf::from(format!("./target/outpost/{}.bin", application_id()));
        let outposts = load_outposts(&file_name).unwrap();
        let values = solve(outposts, moved_materials, days);
        let response = solution_table(outpost_name.clone(), values);
        format!(
            "To maximize total value for {} meeting the {} {} material requirements harvest the following:\n{}",
            outpost_name,
            materials.len(),
            type_option,
            response,
        )
    }

    pub fn handle_outpost(&self, command: ApplicationCommandInteraction) -> String {
        let outpost_name = Self::option_string(&command, "outpost_name");
        let outpost_system = Self::option_string(&command, "outpost_system");
        let capsuleer_name = Self::option_string(&command, "capsuleer_name");
        let corporation_name = Self::option_string(&command, "corporation_name");
        let alliance_name = Self::option_string(&command, "alliance_name");
        let _key = std::env::var("APP_ID");
    
        let outpost = create_outpost(
            &outpost_name,
            &outpost_system,
            &capsuleer_name,
            &corporation_name,
            &alliance_name,
            &application_id(),
        );
    
        format!("Outpost: {}", outpost.name).to_owned()
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

    fn option_type(&self, _interaction: &ApplicationCommandInteraction, type_option: &str) -> Vec<Material> {
        match type_option {
            "ship" => self.ship_materials.lock().unwrap().clone(),
            "structure" => self.structure_materials.lock().unwrap().clone(),
            "corporation" => self.corporation_materials.lock().unwrap().clone(),
            _ => self.materials.lock().unwrap().clone(),
        }
    }
}