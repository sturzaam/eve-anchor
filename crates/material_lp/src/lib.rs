pub mod assertions;
pub mod data;
pub mod objective;
pub mod problem;
pub mod resource;

use std::collections::HashMap;
use good_lp::solvers::Solution;

use objective::{map_objective, map_constellation};
use problem::{ResourceHarvestProblem};
use resource::{Material, CelestialResource};
use manager::database::DatabaseConnection;
use manager::entities::outpost;
use manager::entities::prelude::*;
use manager::*;

pub fn outposts_per_constellation(outposts: &Vec<outpost::Model>) -> Result<Vec<(String, i32)>, anyhow::Error> {
    let mut outpost_counts: HashMap<String, i32> = HashMap::new();
    for outpost in outposts {
        *outpost_counts.entry(outpost.name.to_string()).or_insert(0) += 1;
    }
    Ok(outpost_counts.into_iter().collect())
}

pub async fn create_outpost(
    db: &DatabaseConnection,
    outpost_name: &str,
    outpost_system: &str,
    capsuleer_name: &str,
) -> outpost::Model {
    let alliance = new_alliance(&db, "KEN")
        .await
        .expect("Failed to add alliance to database");
    let corporation = new_corporation(&db, "REEF", alliance.last_insert_id)
        .await
        .expect("Failed to add corporation to database");
    let member = new_member(&db, "eve-anchor", corporation.last_insert_id)
        .await
        .expect("Failed to add member to database");
    let capsuleer = new_capsuleer(&db, capsuleer_name, member.last_insert_id, corporation.last_insert_id)
        .await
        .expect("Failed to add capsuleer to database");
    let problem = new_problem(&db, "Fortizar", vec![], member.last_insert_id, corporation.last_insert_id, Some(alliance.last_insert_id))
        .await
        .expect("Failed to add problem to database");
    let _outpost = new_outpost(&db, outpost_name, outpost_system, 12, 26, capsuleer.last_insert_id, Some(problem.last_insert_id))
        .await
        .expect("Failed to add outpost to database");
    Outpost::find_by_name(outpost_name, &db)
        .await
        .unwrap()
        .unwrap()
}

pub fn solve_for_constellation(
    outposts: Vec<outpost::Model>,
    materials: Vec<Material>,
    days: f64,
) -> Result<Vec<(CelestialResource, f64)>, String> {
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

    let best_production = harvest.best_production()?;

    let resource_quantities: Vec<_> = variables.iter().map(|&v| best_production.value(v)).collect();
    let result: Vec<_> = celestial_resources
        .iter()
        .zip(resource_quantities.iter().cloned())
        .map(|(resource, quantity)| (resource.clone(), quantity))
        .collect();

    Ok(result)
}