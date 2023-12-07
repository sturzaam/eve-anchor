pub mod assertions;
pub mod data;
pub mod objective;
pub mod problem;
pub mod resource;
pub mod manager;

use std::collections::HashMap;
use good_lp::solvers::Solution;

use objective::{map_objective, map_constellation};
use problem::{ResourceHarvestProblem};
use resource::{Material, CelestialResource};
use manager::{Alliance, Corporation, Member, Capsuleer, Outpost};

pub fn outposts_per_constellation(outposts: &Vec<Outpost>) -> Result<Vec<(String, i32)>, anyhow::Error> {
    let mut outpost_counts: HashMap<String, i32> = HashMap::new();
    for outpost in outposts {
        *outpost_counts.entry(outpost.name.to_string()).or_insert(0) += 1;
    }
    Ok(outpost_counts.into_iter().collect())
}

pub fn create_outpost(
    outpost_name: &str,
    outpost_system: &str,
    capsuleer_name: &str,
) -> Outpost {
    Outpost::new(
        outpost_name.to_string(),
        create_capsuleer(capsuleer_name),
        outpost_system.to_string(),
        12,
        22
    ).unwrap()
}

pub fn create_capsuleer(capsuleer_name: &str) -> Capsuleer {
    Capsuleer::new(
        capsuleer_name.to_string(),
        -1,
        20,
        4
    )
}

pub fn create_member(
    member_name: &str,
    corporation_name: &str,
    alliance_name: &str
) -> Member {
    Member::new(
        member_name.to_string(),
        &create_corporation(corporation_name, alliance_name)
    ).unwrap()
}


pub fn create_corporation(
    corporation_name: &str,
    alliance_name: &str
) -> Corporation {
    Corporation::new(corporation_name.to_string(),&create_alliance(alliance_name)).unwrap()
}

pub fn create_alliance(alliance_name: &str) -> Alliance {
    Alliance::new(alliance_name.to_string()).unwrap()
}

pub fn solve_for_constellation(outposts: Vec<Outpost>, materials: Vec<Material>, days: f64) -> Vec<(CelestialResource, f64)> {
    let outpost_count = outposts.len() as f64;
    let (minimum_output, value) = map_objective(materials);
    let (available_key, available_planet, celestial_resources) = map_constellation(outposts);
    let mut harvest = ResourceHarvestProblem::new(
        available_key,
        available_planet,
        minimum_output,
        value,
        days,
    );
    let variables: Vec<_> = celestial_resources
        .clone()
        .into_iter()
        .map(|r| harvest.add_resource(r))
        .collect();
    
    harvest.add_fuel(42002000014, 13., 18000., outpost_count);
    let solution = harvest.best_production();
    let resource_quantities: Vec<_> = variables.iter().map(|&v| solution.value(v)).collect();
    let result: Vec<_> = celestial_resources
        .iter()
        .zip(resource_quantities.iter().cloned())
        .map(|(resource, quantity)| (resource.clone(), quantity))
        .collect();
    
    result
}