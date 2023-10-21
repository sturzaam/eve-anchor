use tabled::{Tabled};
use std::cmp::Ordering;
use crate::CelestialResource;
use crate::data::{get_celestial, system_by_planet, get_item};

#[derive(Tabled)]
pub struct SolutionTable {
    pub outpost_name: Box<str>,
    pub system: Box<str>,
    pub planet: i64,
    pub resource: Box<str>,
    pub array_quantity: f64,
    pub init_output: f64,
}

pub fn solution_table(values: Vec<(CelestialResource, f64)>) -> Vec<SolutionTable> {
    let mut solution_table: Vec<SolutionTable> = Vec::new();

    for (celestial_resource, value) in values.iter() {
        if round_to_2_decimal_places(*value) == 0.0 {
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
            outpost_name: celestial_resource.outpost_name.clone().into(),
            system: system.clone(),
            planet,
            resource,
            array_quantity: round_to_2_decimal_places(*value),
            init_output: round_to_2_decimal_places(celestial_resource.init_output),
        };

        solution_table.push(solution_table_entry);
    }

    solution_table.sort_by(|a, b| {
        let constellation_comparison = a.outpost_name.cmp(&b.outpost_name);
        if constellation_comparison == Ordering::Equal {
            let system_comparison = a.system.cmp(&b.system);
            if system_comparison == Ordering::Equal {
                a.planet.cmp(&b.planet)
            } else {
                system_comparison
            }
        } else {
            constellation_comparison
        }
    });

    solution_table
}

fn round_to_2_decimal_places(value: f64) -> f64 {
    let multiplier = 100.0; // 10^2
    (value * multiplier).round() / multiplier
}