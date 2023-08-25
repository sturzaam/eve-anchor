use std::cmp::Ordering;
use core::str::Split;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use prettytable::{Table, Row, Cell, format::Alignment, format::FormatBuilder,row};
use good_lp::solvers::Solution;
use material_lp::{
    Material, map_objective, 
    data::{CelestialResource, find_item, get_item, find_constellation, get_celestial, system_by_planet},
    solver::{ResourceHarvestProblem}
};

pub struct SolutionTable {
    pub celestial: String,
    pub resource: Box<str>,
    pub arrays: f64,
}

#[test]
fn pretty_requirements_table() {
    let requirements_value = "ID    Names    Quantity    Valuation \\n1    Lustering Alloy    4    10100069.2 \\n2    Smartfab Units    1    398.53";
    let materials = parse_requirements(requirements_value.to_string());
    let pretty_table = material_table(materials.unwrap());
    assert_eq!(pretty_table, "```
 Name             Quantity  Valuation 
 Lustering Alloy         4    10.100M 
 Smartfab Units          1     398.53 

```");
}

pub fn material_table(requirements: Vec<Material>) -> String {
    let mut table = Table::new();
    table.set_format(FormatBuilder::new().padding(1, 1).build());

    // Add headers
    table.add_row(row![b => "Name", "Quantity", "Valuation"]);

    // Add data rows
    for material in &requirements {
        table.add_row(Row::new(vec![
            Cell::new_align(&material.name, Alignment::LEFT),
            Cell::new_align(&format_value(material.quantity as f64), Alignment::RIGHT),
            Cell::new_align(&format_value(material.valuation), Alignment::RIGHT),
        ]));
    }
    let table_string = table.to_string();
    let formatted_output = match table_string.get(..1999) {
        Some(substring) => format!("```\n{}\n```", substring),
        None => format!("```\n{}\n```", table_string),
    };
    formatted_output
}

fn format_value(value: f64) -> String {
    const BILLION: f64 = 1_000_000_000.0;
    const MILLION: f64 = 1_000_000.0;
    const THOUSAND: f64 = 1_000.0;

    if value >= BILLION {
        format!("{:.3}B", value / BILLION)
    } else if value >= MILLION {
        format!("{:.3}M", value / MILLION)
    } else if value >= THOUSAND {
        format!("{:.3}K", value / THOUSAND)
    } else {
        value.to_string()
    }
}

#[test]
fn pretty_solution_table() {
    let constellation = "KUSW-P";
    let solution: Vec<(CelestialResource, f64)> = vec![
        (CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000005,
            resource_type_id: 42001000032,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 22.),
        (CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000005,
            resource_type_id: 42001000033,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 3.),
        (CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000007,
            resource_type_id: 42001000032,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 21.),
        (CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000007,
            resource_type_id: 42001000033,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 2.),
    ];
    let pretty_table = solution_table(constellation, solution);
    assert_eq!(pretty_table, "```
Celestial Resource       Arrays
Tanoo 2   Silicate Glass 22
Tanoo 2   Smartfab Units 3
Tanoo 3   Silicate Glass 21
Tanoo 3   Smartfab Units 2

```");

}

pub fn solution_table(constellation: &str, values: Vec<(CelestialResource, f64)>) -> String {
    let mut solution_table: Vec<SolutionTable> = Vec::new();

    for (celestial_resource, value) in values.iter() {
        if round_to_2_decimal_places(*value) == 0.0 {
            continue;
        }
        let constellation_id = find_constellation(constellation).expect("Constellation not found");
        if *constellation_id != celestial_resource.constellation_id {
            continue;
        }
        let system = system_by_planet(celestial_resource.planet_id)
            .unwrap()
            .en_name
            .clone();
        let planet = get_celestial(celestial_resource.planet_id)
            .unwrap()
            .celestial_index
            .clone();
        let resource = get_item(celestial_resource.resource_type_id)
            .unwrap()
            .en_name
            .clone();
        
        let solution_table_entry = SolutionTable {
            celestial: format!("{} {}", system, planet), 
            resource,
            arrays: round_to_2_decimal_places(*value),
        };

        solution_table.push(solution_table_entry);
    }

    solution_table.sort_by(|a, b| {
        let celestial_comparison = a.celestial.cmp(&b.celestial);
        if celestial_comparison == Ordering::Equal {
            let resource_comparison = a.resource.cmp(&b.resource);
            if resource_comparison == Ordering::Equal {
                a.arrays.partial_cmp(&b.arrays).unwrap_or(Ordering::Equal)
            } else {
                resource_comparison
            }
        } else {
            celestial_comparison
        }
    });
    
    
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 0)
            .build()
    );
    table.add_row(Row::new(vec![
        Cell::new("Celestial").style_spec("bFg"),
        Cell::new("Resource").style_spec("bFg"),
        Cell::new("Arrays").style_spec("bFg"),
    ]));

    for item in solution_table {
        table.add_row(Row::new(vec![
            Cell::new(&item.celestial),
            Cell::new(&item.resource),
            Cell::new(&item.arrays.to_string()),
        ]));
    }

    let table_string = table.to_string();
    let formatted_output = match table_string.get(..1999) {
        Some(substring) => format!("```\n{}\n```", substring),
        None => format!("```\n{}\n```", table_string),
    };
    formatted_output
}

fn round_to_2_decimal_places(value: f64) -> f64 {
    let multiplier = 100.0; // 10^2
    (value * multiplier).round() / multiplier
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

pub fn parse_constellations(requirements: Option<CommandDataOption>) -> Result<Vec<(String, i32)>, String> {
    let mut constellations: Vec<(String, i32)> = Vec::new();
    let constellations_value = requirements
        .unwrap()
        .value
        .unwrap()
        .to_string();
    let constellation_trimed = constellations_value.trim_matches(['"'].as_ref());

    for constellation in constellation_trimed.split_whitespace() {
        let pos = constellation
            .find('=')
            .ok_or_else(|| format!("Invalid KEY=value: no `=` found in `{}`", constellation))?;
        let key = constellation[..pos].to_string();
        let value = constellation[pos + 1..].parse().map_err(|_| format!("Invalid value: {}", constellation))?;
        constellations.push((key, value));
    }

    Ok(constellations)
}

pub fn solve_resource_problem(materials: Vec<Material>, days: f64, constellations: Vec<(String, i32)>) -> Vec<(CelestialResource, f64)> {
    let available_outposts = constellations.iter().map(|(_, value)| value).sum();
    let (available_constellation, available_planet, minimum_output, celestial_resources, value) = map_objective(materials, constellations);
    let mut harvest = ResourceHarvestProblem::new(
        available_constellation,
        available_planet,
        minimum_output,
        value,
        available_outposts,
        days,
    );
    let variables: Vec<_> = celestial_resources
        .clone()
        .into_iter()
        .map(|r| harvest.add_resource(r))
        .collect();
    let solution = harvest.best_production();
    let resource_quantities: Vec<_> = variables
        .iter()
        .map(|&v| solution.value(v))
        .collect();
    let results = celestial_resources
        .into_iter()
        .zip(resource_quantities)
        .collect();
    results
}
