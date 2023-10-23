pub mod outpost;
pub mod report;

use std::fs::File;
use std::io::{Read};
use std::path::Path;

use std::cmp::Ordering;
use core::str::Split;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use prettytable::{Table, Row, Cell, format::Alignment, format::FormatBuilder,row};
use good_lp::solvers::Solution;
use material_lp::{
    objective, solve,
    resource::{Material, CelestialResource},
    data::{find_item, get_item, find_constellation, get_celestial, system_by_planet},
    problem::{ResourceHarvestProblem},
    structure::{Capsuleer, Outpost, Corporation, Alliance},
};

pub fn load_outposts(file_path: &Path) -> Result<Vec<Outpost>, anyhow::Error> {
    let mut outposts: Vec<Outpost> = Vec::new();
    if file_path.exists() {
        let mut file = File::open(file_path).expect(&format!("Failed to open: {}", file_path.display()));
        let mut corporation_data = Vec::new();
        file.read_to_end(&mut corporation_data).expect(&format!("Failed to read: {}", file_path.display()));
        let corporation: Corporation = bincode::deserialize(&corporation_data).expect(&format!("Failed to deserialize: {}", file_path.display()));
        outposts = corporation.outposts.into_iter().collect()
    }

    Ok(outposts)
}

pub fn application_id() -> String {
    match std::env::var("APP_ID") {
        Ok(value) => value,
        Err(_) => "test".to_string(),
    }
}

#[test]
fn parse_newline_four_space_requirements() {
    let requirements_value = "ID    Names    Quantity    Valuation \\n1    Lustering Alloy    4    10100069.2 \\n2    Smartfab Units    1    398.53";
    let materials = parse_requirements(requirements_value.to_string());
    let material = Material {
        resource_type_id: 42001000000,
        name: "Lustering Alloy".into(),
        quantity: 4,
        valuation: 10100069.2
    };
    println!("materials: {:?}", materials);

    let materials_vec = materials.unwrap_or_else(|err| {
        eprintln!("Error occurred: {}", err);
        Vec::<Material>::new()
    });

    println!("materials_vec: {:?}", materials_vec);

    assert!(materials_vec.contains(&material));
}

#[test]
fn parse_doublespace_tab_requirements() {
    let requirements_value = "ID\\tNames\\tQuantity\\tValuation  1\\tLustering Alloy\\t4\\t10100069.2  2\\tSmartfab Units\\t1\\t398.53";
    let materials = parse_requirements(requirements_value.to_string());
    let material = Material {
        resource_type_id: 42001000000,
        name: "Lustering Alloy".into(),
        quantity: 4,
        valuation: 10100069.2
    };
    println!("requirements_value: {:?}", requirements_value);
    println!("materials: {:?}", materials);

    let materials_vec = materials.unwrap_or_else(|err| {
        eprintln!("Error occurred: {}", err);
        Vec::<Material>::new()
    });

    println!("materials_vec: {:?}", materials_vec);

    assert!(materials_vec.contains(&material));
}

pub fn parse_requirements(string: String) -> Result<Vec::<Material>, anyhow::Error> {
    let mut materials = Vec::<Material>::new();
    let mut lines: Split<&str>;

    if string.contains(" \\n") {
        lines = string.split("\\n");
    } else {
        lines = string.split("  ");
    }
    lines.next();
    for line in lines {
        let material_split: Vec<&str>;
        if line.contains("    ") {
            material_split = line.split("    ").collect();
        } else {
            material_split = line.split("\\t").collect();
        }
        if material_split.len() == 4 {
            let resource_type_id = match find_item(material_split[1].into()) {
                Some(id) => *id,
                None => {
                    // Handle the error when find_item returns None
                    eprintln!("Failed to find resource type ID for material: {}", material_split[1]);
                    continue;
                }
            };
            let name = material_split[1].into();
            let quantity = match material_split[2].parse::<i64>() {
                Ok(quantity) => quantity,
                Err(err) => {
                    // Handle the parsing error for quantity
                    eprintln!("Failed to parse quantity for material: {}: {} from {}", material_split[1], err, material_split[2]);
                    continue;
                }
            };
        
            let valuation = match material_split[3].trim_matches([' ', '"'].as_ref()).parse::<f64>() {
                Ok(valuation) => valuation,
                Err(err) => {
                    // Handle the parsing error for valuation
                    eprintln!("Failed to parse valuation for material: {}: {} from {}", material_split[1], err, material_split[3]);
                    // response = format!("Failed to parse valuation for material: {}: {} from {}", material_split[1], err, material_split[3]);
                    continue;
                }
            };
        
            materials.push(Material {
                resource_type_id,
                name,
                quantity,
                valuation,
            });
        }
    }
    Ok(materials)
}

pub fn parse_requirements_option(requirements: Option<CommandDataOption>) -> Vec<Material> {
    let mut materials = Vec::<Material>::new();
    let requirements_value = requirements.unwrap().value.unwrap().to_string();
    let mut lines = requirements_value.split("  ");
    lines.next();
    for line in lines {
        let material_split: Vec<&str> = line.split("\\t").collect();
        if material_split.len() == 4 {
            let resource_type_id = match find_item(material_split[1].into()) {
                Some(id) => *id,
                None => {
                    // Handle the error when find_item returns None
                    eprintln!("Failed to find resource type ID for material: {}", material_split[1]);
                    continue;
                }
            };
            let name = material_split[1].into();
            let quantity = match material_split[2].parse::<i64>() {
                Ok(quantity) => quantity,
                Err(err) => {
                    // Handle the parsing error for quantity
                    eprintln!("Failed to parse quantity for material: {}: {} from {}", material_split[1], err, material_split[2]);
                    continue;
                }
            };
        
            let valuation = match material_split[3].trim_matches([' ', '"'].as_ref()).parse::<f64>() {
                Ok(valuation) => valuation,
                Err(err) => {
                    // Handle the parsing error for valuation
                    eprintln!("Failed to parse valuation for material: {}: {} from {}", material_split[1], err, material_split[3]);
                    // response = format!("Failed to parse valuation for material: {}: {} from {}", material_split[1], err, material_split[3]);
                    continue;
                }
            };
        
            materials.push(Material {
                resource_type_id,
                name,
                quantity,
                valuation,
            });
        }
    }
    materials
}
