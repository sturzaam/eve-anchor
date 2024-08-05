use prettytable::{Table, Row, Cell, format::Alignment, format::FormatBuilder,row};
use std::cmp::Ordering;
use material_lp::data::{get_celestial, system_by_planet, get_item, get_system};
use material_lp::manager::Outpost; 
use material_lp::resource::{Material, CelestialResource}; 


pub struct SolutionTable {
    pub celestial: String,
    pub resource: Box<str>,
    pub arrays: f64,
}

pub struct OutpostTable {
    pub outpost_name: Box<str>,
    pub capsuleer_name: Box<str>,
    pub system: Box<str>,
    pub planets: Box<str>,
    pub arrays: Box<str>,
}

pub fn solution_table(key: String, values: Vec<(CelestialResource, f64)>) -> String {
    let mut solution_table: Vec<SolutionTable> = Vec::new();

    for (celestial_resource, value) in values.iter() {
        if round_to_2_decimal_places(*value) == 0.0 {
            continue;
        }
        if *key != celestial_resource.key {
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

pub fn outpost_table(values: Vec<Outpost>) -> String {
    let mut outpost_table: Vec<OutpostTable> = Vec::new();

    for outpost in values.iter() {
        let moved_outpost = outpost.clone();
        outpost_table.push(
            OutpostTable {
                outpost_name: moved_outpost
                    .name
                    .into(),
                capsuleer_name: moved_outpost
                    .capsuleer
                    .name
                    .into(),
                system: get_system(moved_outpost.system_id)
                    .unwrap()
                    .en_name
                    .clone(),
                planets: moved_outpost
                    .available_planets
                    .to_string()
                    .into(),
                arrays: moved_outpost
                    .available_arrays
                    .to_string()
                    .into(),
            }
        );
    }


    outpost_table.sort_by(|a, b| a.system.cmp(&b.system));
    
    
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 0)
            .build()
    );
    table.add_row(Row::new(vec![
        Cell::new("Outpost").style_spec("bFg"),
        Cell::new("Capsuleer").style_spec("bFg"),
        Cell::new("System").style_spec("bFg"),
        Cell::new("Planets").style_spec("bFg"),
        Cell::new("Arrays").style_spec("bFg"),
    ]));

    for item in outpost_table {
        table.add_row(Row::new(vec![
            Cell::new(&item.outpost_name),
            Cell::new(&item.capsuleer_name),
            Cell::new(&item.system),
            Cell::new(&item.planets),
            Cell::new(&item.arrays),
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

fn round_to_2_decimal_places(value: f64) -> f64 {
    let multiplier = 100.0; // 10^2
    (value * multiplier).round() / multiplier
}