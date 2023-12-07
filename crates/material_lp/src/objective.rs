mod objective {
    use std::collections::HashMap;
    use crate::problem::{Value};
    use crate::data::{
        PLANETS,
        find_item,
        slice_celestials,
        get_constellation
    };
    use crate::resource::{Material, CelestialResource, celestial_resources_by_constellation};
    use crate::manager::Outpost;
    
    pub fn map_objective(materials: Vec<Material>) -> (HashMap<i64, f64>, Value) {
        let mut minimum_output: HashMap<i64, f64> = HashMap::new();
        let mut value = Value::default();
    
        for material in materials {
            let entry = minimum_output.entry(material.resource_type_id).or_insert(0.0);
            *entry += material.quantity as f64;
            match material.name.as_ref() {
                "Lustering Alloy" => {
                    value.lustering_allow = material.valuation / material.quantity as f64;
                },
                "Sheen Compound" => {
                    value.sheen_compound = material.valuation / material.quantity as f64;
                },
                "Gleaming Alloy" => {
                    value.gleaming_alloy = material.valuation / material.quantity as f64;
                },
                "Motley Compound" => {
                    value.motley_compound = material.valuation / material.quantity as f64;
                },
                "Precious Alloy" => {
                    value.precious_alloy = material.valuation / material.quantity as f64;
                },
                "Condensed Alloy" => {
                    value.condensed_alloy = material.valuation / material.quantity as f64;
                },
                "Fiber Composite" => {
                    value.fiber_composite = material.valuation / material.quantity as f64;
                },
                "Lucent Compound" => {
                    value.lucent_compound = material.valuation / material.quantity as f64;
                },
                "Opulent Compound" => {
                    value.opulent_compound = material.valuation / material.quantity as f64;
                },
                "Glossy Compound" => {
                    value.glossy_compound = material.valuation / material.quantity as f64;
                },
                "Reactive Gas" => {
                    value.reactive_gas = material.valuation / material.quantity as f64;
                },
                "Noble Gas" => {
                    value.noble_gas = material.valuation / material.quantity as f64;
                },
                "Crystal Compound" => {
                    value.crystal_compound = material.valuation / material.quantity as f64;
                },
                "Dark Compound" => {
                    value.dark_compound = material.valuation / material.quantity as f64;
                },
                "Base Metals" => {
                    value.base_metals = material.valuation / material.quantity as f64;
                },
                "Heavy Metals" => {
                    value.heavy_metals = material.valuation / material.quantity as f64;
                },
                "Toxic Metals" => {
                    value.toxic_metals = material.valuation / material.quantity as f64;
                },
                "Industrial Fibers" => {
                    value.industrial_fibers = material.valuation / material.quantity as f64;
                },
                "Noble Metals" => {
                    value.noble_metals = material.valuation / material.quantity as f64;
                },
                "Reactive Metals" => {
                    value.reactive_metals = material.valuation / material.quantity as f64;
                },
                "Supertensile Plastics" => {
                    value.supertensile_plastics = material.valuation / material.quantity as f64;
                },
                "Polyaramids" => {
                    value.polyaramids = material.valuation / material.quantity as f64;
                },
                "Construction Blocks" => {
                    value.construction_blocks = material.valuation / material.quantity as f64;
                },
                "Nanites" => {
                    value.nanites = material.valuation / material.quantity as f64;
                },
                "Coolant" => {
                    value.coolant = material.valuation / material.quantity as f64;
                },
                "Condensates" => {
                    value.condensates = material.valuation / material.quantity as f64;
                },
                "Silicate Glass" => {
                    value.silicate_glass = material.valuation / material.quantity as f64;
                },
                "Smartfab Units" => {
                    value.smartfab_units = material.valuation / material.quantity as f64;
                },
                "Suspended Plasma" => {
                    value.suspended_plasma = material.valuation / material.quantity as f64;
                },
                "Heavy Water" => {
                    value.heavy_water = material.valuation / material.quantity as f64;
                },
                "Plasmoids" => {
                    value.plasmoids = material.valuation / material.quantity as f64;
                },
                "Liquid Ozone" => {
                    value.liquid_ozone = material.valuation / material.quantity as f64;
                },
                "Ionic Solutions" => {
                    value.ionic_solutions = material.valuation / material.quantity as f64;
                },
                "Oxygen Isotopes" => {
                    value.oxygen_isotopes = material.valuation / material.quantity as f64;
                },
                _ => ()//panic!("Invalid attribute name: {}", material.name.as_ref()),
            }
        }
        
        (minimum_output, value)
    }
    
    pub fn map_constellation(outposts: Vec<Outpost>) -> (HashMap<String, i32>, HashMap<i64, i32>, Vec<CelestialResource>) {
        let mut available_constellation: HashMap<String, i32> = HashMap::new();
        let mut available_planet: HashMap<i64, i32> = HashMap::new();
        let mut available_celestial_resource: Vec<CelestialResource> = Vec::new();
        
        for outpost in outposts {    
            let constellation = get_constellation(outpost.constellation_id);
            match available_planets_by_outpost(outpost.clone(), outpost.available_arrays) {
                Ok(planets) => {
                    *available_constellation
                        .entry(constellation.unwrap().en_name.to_string())
                        .or_insert(0) += outpost.available_arrays * outpost.available_planets;
                    for (key, _value) in planets {
                        *available_planet.entry(key).or_insert(0) += outpost.available_arrays;
                    }
                    available_celestial_resource.extend(celestial_resources_by_constellation(outpost.constellation_id))
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
        
        (available_constellation, available_planet, available_celestial_resource)
    }
    
    pub fn push_material(line: &str, materials: &mut Vec<Material>) {
        let material_split: Vec<&str> = line.split("\t").collect();
        if material_split.len() >= 4 {
            materials.push(
                Material {
                    resource_type_id: *find_item(material_split[1].into()).unwrap(),
                    name: material_split[1].into(),
                    quantity: material_split[2].parse::<i64>().unwrap(),
                    valuation: material_split[3].trim().parse::<f64>().unwrap(),
                }
            );
        }
    }

    pub fn parse_decomposed_list(text: &str) ->  Result<Vec::<Material>, anyhow::Error> {
        let expected_header = "ID\tNames\tQuantity\tValuation ";
        let mut materials = Vec::<Material>::new();
        let mut lines = text.lines();
    
        if let Some(first_line) = lines.next() {
            if first_line != expected_header {
                return Err(anyhow::anyhow!("Invalid header line.").into());
            }
            for line in lines {
                push_material(line, &mut materials);
            }
        } else {
            return Err(anyhow::anyhow!("No header line.").into());
        }
        Ok(materials)
    }

    pub fn available_planets_by_outpost(outpost: Outpost, number: i32) -> Result<HashMap<i64, i32>, Box<dyn std::error::Error>> {
        let celestials = slice_celestials(outpost.constellation_id).expect("Failed to slice celestials");
        let available_planets: HashMap<i64, i32> = PLANETS
            .iter()
            .filter(|(key, _)| celestials.contains_key(*key))
            .map(|(key, _)| (*key, number))
            .collect();

        Ok(available_planets)
    }
}


pub use objective::{map_objective, map_constellation, parse_decomposed_list, push_material};