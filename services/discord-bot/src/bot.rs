use std::sync::{Arc, Mutex};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use crate::lib::{parse_requirements, solve_resource_problem, parse_constellations, solution_table, material_table};
use material_lp::{Material};

pub struct Bot {
    pub materials: Arc<Mutex<Vec<Material>>>,
    pub ship_materials: Arc<Mutex<Vec<Material>>>,
    pub structure_materials: Arc<Mutex<Vec<Material>>>,
    pub corporation_materials: Arc<Mutex<Vec<Material>>>,
    pub constellations: Arc<Mutex<Vec<(String, i32)>>>,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            materials: Arc::new(Mutex::new(Vec::new())),
            ship_materials: Arc::new(Mutex::new(Vec::new())),
            structure_materials: Arc::new(Mutex::new(Vec::new())),
            corporation_materials: Arc::new(Mutex::new(Vec::new())),
            constellations: Arc::new(Mutex::new(Vec::new())),
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

    pub fn add_constellation(&self, constellation: (String, i32)) {
        let mut constellations = self.constellations.lock().unwrap();
        constellations.push(constellation);
    }

    pub fn clear_constellations(&self) {
        let mut constellations = self.constellations.lock().unwrap();
        constellations.clear();
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
        let mut material_requirements: Vec<Material> = Vec::new();
        match clean_type {
            "ship" => {
                material_requirements = parse_requirements(requirements_string.clone())
                    .expect("Failed to parse ship requirements.");
                self.clear_ship_materials();
                material_requirements
                    .iter()
                    .for_each(|material| self.add_ship_material(material.clone()));
                format!("{} Ship materials configured",
                    self.ship_materials.lock().unwrap().clone().len(),
                )
            },
            "structure" => {
                material_requirements = parse_requirements(requirements_string.clone())
                    .expect("Failed to parse material requirements.");
                self.clear_structure_materials();
                material_requirements
                    .iter()
                    .for_each(|material| self.add_structure_material(material.clone()));
                format!("{} Structure materials configured",
                    self.structure_materials.lock().unwrap().clone().len(),
                )
            },
            "corporation" => {
                material_requirements = parse_requirements(requirements_string.clone())
                    .expect("Failed to parse corporation requirements.");
                self.clear_corporation_materials();
                material_requirements
                    .iter()
                    .for_each(|material| self.add_corporation_material(material.clone()));
                format!("{} Corporation materials configured",
                    self.corporation_materials.lock().unwrap().clone().len(),
            )
            },
            "material" => {
                material_requirements = parse_requirements(requirements_string.clone())
                    .expect("Failed to parse material requirements.");
                self.clear_materials();
                material_requirements
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

    pub fn handle_constellations(&self, command: ApplicationCommandInteraction) -> String {
        let requirements = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "requirements")
            .cloned();
        let mut response = "".to_owned();
        self.clear_constellations();
        if let Some(constellations) = match parse_constellations(requirements.clone()) {
            Ok(constellations) => Some(constellations),
            Err(err) => {
                eprintln!("Failed to parse constellations: {}", err);
                response = format!("Failed to parse constellations: {}", err);
                None
            }
        } {
            for constellation in constellations {
                self.add_constellation(constellation.clone());
            }
            "Constellations configured".to_owned()
        } else {
            response.to_owned()
        }
    }

    pub fn handle_material(&self, command: ApplicationCommandInteraction) -> String {
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
        let mut material_requirements: Vec<Material> = Vec::new();
        match clean_type {
            "ship" => {
                material_requirements = self.ship_materials.lock().unwrap().clone();
            },
            "structure" => {   
                material_requirements = self.structure_materials.lock().unwrap().clone();
            },
            "corporation" => {
                material_requirements = self.corporation_materials.lock().unwrap().clone();
            },
            _ => {
                material_requirements = self.materials.lock().unwrap().clone();
            }
        }
        material_table(material_requirements)
    }
    
    pub fn handle_solver(&self, command: ApplicationCommandInteraction) -> String {
        let day_option = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "days")
            .cloned();
        let days: f64 = day_option.unwrap()
            .value
            .unwrap()
            .as_f64()
            .unwrap();
        let constellation_option = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "constellation")
            .cloned();
        let dirty_constellation = match constellation_option {
            Some(c) => c.value.unwrap().to_string(),
            None => "material".to_string(),
        };
        let clean_constellation = dirty_constellation.trim_matches('\"');
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
        let mut material_requirements: Vec<Material> = Vec::new();
        let requirements = command
            .data
            .options
            .iter()
            .find(|opt| opt.name == "requirements")
            .cloned();
        match requirements {
            Some(requirements) => {
                let requirements_string = Some(requirements).unwrap().value.unwrap().to_string();
                self.clear_materials();
                parse_requirements(requirements_string)
                    .expect("Failed to parse requirements.")
                    .iter()
                    .for_each(|material| self.add_material(material.clone()));
            },
            None => (),
        }
        match clean_type {
            "ship" => {
                material_requirements = self.ship_materials.lock().unwrap().clone();
            },
            "structure" => {   
                material_requirements = self.structure_materials.lock().unwrap().clone();
            },
            "corporation" => {
                material_requirements = self.corporation_materials.lock().unwrap().clone();
            },
            _ => {
                material_requirements = self.materials.lock().unwrap().clone();
            }
        }
        let materials = material_requirements.clone();
        let constellations = self.constellations.lock().unwrap().clone();
        let celestial_resource_values = solve_resource_problem(materials, days, constellations);
        let mut response = solution_table(clean_constellation, celestial_resource_values);
        format!(
            "To maximize total value in constellation {} meeting the {} {} material requirements harvest the following:\n{}",
            clean_constellation,
            material_requirements.clone().len(),
            clean_type,
            response,
        )
    }
}