pub mod assertions;
pub mod data;
pub mod objective;
pub mod problem;
pub mod resource;
pub mod solution;
pub mod structure;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::path::Path;
use good_lp::solvers::Solution;

use objective::{map_outpost, map_objective};
use problem::ResourceHarvestProblem;
use resource::{Material, CelestialResource};
use structure::{Capsuleer, Outpost, Corporation, Alliance};

pub fn load_outposts(file_path: &Path) -> Result<Vec<(String, i32)>, anyhow::Error> {
    let mut outpost_counts: HashMap<String, i32> = HashMap::new();
    if file_path.exists() {
        let mut file = File::open(file_path).expect(&format!("Failed to open: {}", file_path.display()));
        let mut corporation_data = Vec::new();
        file.read_to_end(&mut corporation_data).expect(&format!("Failed to read: {}", file_path.display()));
        let corporation: Corporation = bincode::deserialize(&corporation_data).expect(&format!("Failed to deserialize: {}", file_path.display()));
        
        for outpost in &corporation.outposts {
            *outpost_counts.entry(outpost.name.to_string()).or_insert(0) += 1;
        }
    }

    Ok(outpost_counts.into_iter().collect())
}

pub fn create_outpost(
    outpost_name: &str,
    outpost_system: &str,
    capsuleer_name: &str,
    corporation_name: &str,
    alliance_name: &str,
    key: &str,
) -> Outpost {
    Outpost::new(
        outpost_name.to_string(),
        create_capsuleer(capsuleer_name, corporation_name, alliance_name),
        outpost_system.to_string(),
        key.to_string(),
    ).unwrap()
}

pub fn create_capsuleer(
    capsuleer_name: &str,
    corporation_name: &str,
    alliance_name: &str,
) -> Capsuleer {
    Capsuleer::new(
        capsuleer_name.to_string(),
        create_corporation(corporation_name, alliance_name),
        -1,
        20,
        4
    )
}

pub fn create_corporation(
    corporation_name: &str,
    alliance_name: &str
) -> Corporation {
    Corporation {
        name: corporation_name.to_string(),
        alliance: create_alliance(alliance_name),
        outposts: vec![],
    }
}

pub fn create_alliance(alliance_name: &str) -> Alliance {
    Alliance {
        name: alliance_name.to_string(),
        corporations: vec![],
    }
}

pub fn solve(outposts: Vec<Outpost>, materials: Vec<Material>, days: f64) -> Vec<(CelestialResource, f64)> {
    let (minimum_output, value) = map_objective(materials);
    let (available_outpost, available_planet, celestial_resources) = map_outpost(outposts);
    let mut harvest = ResourceHarvestProblem::new(
        available_outpost,
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
    let solution = harvest.best_production();
    let resource_quantities: Vec<_> = variables.iter().map(|&v| solution.value(v)).collect();
    let result: Vec<_> = celestial_resources
        .iter()
        .zip(resource_quantities.iter().cloned())
        .map(|(resource, quantity)| (resource.clone(), quantity))
        .collect();
    
    result
}