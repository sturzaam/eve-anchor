pub mod data;
pub mod solver;
pub mod assertions;

use solver::{Value};
use std::collections::HashMap;
use arboard::Clipboard;
use anyhow::Context;
use data::{find_item, available_planets_by_constellation, find_constellation, celestial_resources_by_constellation};
use data::{CelestialResource};

#[test]
fn mapped_objective() {
    let mut clipboard = Clipboard::new().unwrap();
    let the_string = "ID	Names	Quantity	Valuation 
    1	Lustering Alloy	1	167.09 
    2	Sheen Compound	2	400.00 
    ";
    let _ = clipboard.set_text(the_string);
    let materials = parse_decomposed_list();
    let constellations = vec![
        ("FY6-NK".to_string(), 3),
        ("E-ILCH".to_string(), 3),
        ("42-WDG".to_string(), 3),
        ("R2-BT6".to_string(), 3),
        ("IZ-FBT".to_string(), 3),
        ("I-CCEI".to_string(), 3),
    ];
    let (_, _, required, _, value) = map_objective(materials.unwrap(), constellations);
    assert_eq!(required.get(&42001000000), Some(&1.));
    assert_eq!(value.lustering_allow, 167.09);
    assert_eq!(required.get(&42001000001), Some(&2.));
    assert_eq!(value.sheen_compound, 200.00);
}

pub fn map_objective(materials: Vec<Material>, constellations: Vec<(String, i32)>) -> (HashMap<i64, f64>, HashMap<i64, f64>, HashMap<i64, f64>, Vec<CelestialResource>, Value) {
    let mut available_constellation: HashMap<i64, f64> = HashMap::new();
    let mut available_planet: HashMap<i64, f64> = HashMap::new();
    let mut celestial_resources: Vec<CelestialResource> = Vec::new();
    let mut minimum_output: HashMap<i64, f64> = HashMap::new();
    let mut value = Value::default();
    
    for (constellation, outposts) in constellations {
        let available_array = outposts as f64 * 22.;
        match available_planets_by_constellation(&constellation, available_array) {
            Ok(planets) => {
                *available_constellation
                    .entry(*find_constellation(&constellation).expect("Constellation not found"))
                    .or_insert(available_array * 12.) = available_array * 12.;
                available_planet.extend(planets);
                celestial_resources.extend(celestial_resources_by_constellation(&constellation))
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

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
    
    (available_constellation, available_planet, minimum_output, celestial_resources, value)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub resource_type_id: i64,
    pub name: Box<str>,
    pub quantity: i64,
    pub valuation: f64,
}

#[test]
fn push_a_material() {
    let mut result = Vec::<Material>::new();
    push_material("1	Lustering Alloy	4	10100069.2 ", &mut result);
    let materials: Vec<Material> = vec![
        Material {
            resource_type_id: 42001000000,
            name: "Lustering Alloy".into(),
            quantity: 4,
            valuation: 10100069.2
        }
    ];
    assert_eq!(result, materials);
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

#[test]
fn parse_invalid_header() {
    let mut clipboard = Clipboard::new().unwrap();
    let _ = clipboard.set_text("");
    let materials = parse_decomposed_list();
    assert!(materials.is_err());
    assert!(materials.unwrap_err().to_string().contains("No header line."));
}

#[test]
fn parse_a_decomposed_list() {
    let mut clipboard = Clipboard::new().unwrap();
    let the_string = "ID	Names	Quantity	Valuation 
    1	Lustering Alloy	4	10100069.2 
    2	Liquid Ozone	1	171.78 
    ";
    let _ = clipboard.set_text(the_string);
    let materials = parse_decomposed_list();
    let material = Material {
        resource_type_id: 42001000000,
        name: "Lustering Alloy".into(),
        quantity: 4,
        valuation: 10100069.2
    };
    assert!(materials.unwrap().contains(&material));
}

pub fn parse_decomposed_list() ->  Result<Vec::<Material>, anyhow::Error> {
    let expected_header = "ID\tNames\tQuantity\tValuation ";
    let mut materials = Vec::<Material>::new();
    let mut clipboard = Clipboard::new().context("Failed to create clipboard.")?;
    let text = clipboard.get_text().context("Failed to get clipboard text.")?;
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