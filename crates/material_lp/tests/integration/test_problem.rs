use material_lp::{create_outpost, solve_for_constellation};
use material_lp::objective::{
    map_objective,
    map_constellation,
    parse_decomposed_list
};
use material_lp::resource::{CelestialResource};
use material_lp::problem::{ResourceHarvestProblem};

use crate::DatabaseManager;
use manager::environment::EnvironmentManager;

#[tokio::test]
async fn fuel_problem() {
    let config = EnvironmentManager::load_config("test")
        .await
        .expect("Failed to load configuration");
    let db = DatabaseManager::revision(&config)
        .await
        .expect("Failed to connect to database");
    let outposts = vec![
        create_outpost(&db, "Outpost1", "Tanoo", "Aaron").await,
        create_outpost(&db, "Outpost2", "Sooma", "Benjamin").await,
        create_outpost(&db, "Outpost3", "Futzchag", "Caroline").await,
        create_outpost(&db, "Outpost4", "Fovihi", "David").await,
        create_outpost(&db, "Outpost5", "Mohas", "Emily").await,
        create_outpost(&db, "Outpost6", "Dooz", "Fiona").await,
    ];
    let materials = parse_decomposed_list("ID	Names	Quantity	Valuation 
    1	Silicate Glass	1	1011.34 
    2	Smartfab Units	1	418.3 
    3	Liquid Ozone	1	166.13 
    4	Reactive Gas	1	195.65 
    5	Noble Gas	1	363.2 
    6	Industrial Fibers	1	1199.78 
    7	Supertensile Plastics	1	512.55 
    8	Polyaramids	1	102.93 
    9	Coolant	1	607.45 
    10	Condensates	1	346.7 
    11	Construction Blocks	1	381.78 
    12	Nanites	1	1448.58 
    ").unwrap();
    let (minimum_output, value) = map_objective(materials);
    let outpost_count = outposts.len() as f64;
    let (available_key, available_planet, _celestial_resources) = map_constellation(outposts);
    let mut harvest = ResourceHarvestProblem::new(
        available_key,
        available_planet,
        minimum_output.clone(),
        value,
        7.,
    );
    harvest.add_fuel(42002000014, 13., 18000., outpost_count);
    assert_eq!(harvest.minimum_output.get(&42002000014), Some(&1395693.3076923075));

}


#[tokio::test]
async fn using_constellation() {
    let config = EnvironmentManager::load_config("test")
        .await
        .expect("Failed to load configuration");
    let db = DatabaseManager::revision(&config)
        .await
        .expect("Failed to connect to database");
    let outposts = vec![
        create_outpost(&db, "Outpost1", "Tanoo", "Aaron").await,
        create_outpost(&db, "Outpost2", "Sooma", "Benjamin").await,
        create_outpost(&db, "Outpost3", "Futzchag", "Caroline").await,
        create_outpost(&db, "Outpost4", "Fovihi", "David").await,
        create_outpost(&db, "Outpost5", "Mohas", "Emily").await,
        create_outpost(&db, "Outpost6", "Dooz", "Fiona").await,
    ];
    let materials = parse_decomposed_list("ID	Names	Quantity	Valuation 
    1	Silicate Glass	1	1011.34 
    2	Smartfab Units	1	418.3 
    3	Liquid Ozone	1	166.13 
    4	Reactive Gas	1	195.65 
    5	Noble Gas	1	363.2 
    6	Industrial Fibers	1	1199.78 
    7	Supertensile Plastics	1	512.55 
    8	Polyaramids	1	102.93 
    9	Coolant	1	607.45 
    10	Condensates	1	346.7 
    11	Construction Blocks	1	381.78 
    12	Nanites	1	1448.58 
    ").unwrap();

    let results = match solve_for_constellation(outposts, materials, 7.) {
        Ok(res) => res,
        Err(err) => {
            // Handle the error or panic with a message
            panic!("Failed to solve problem: {}", err);
        }
    };
    let expected: Vec<(CelestialResource, f64)> = vec![
        (
            CelestialResource { 
                key: "San Matar".into(),
                planet_id: 40000043,
                resource_type_id: 42001000032,
                init_output: 9.859999656677246,
                richness_index: 1,
                richness_value: 1644
            }, 
            26.0
        ),(
            CelestialResource {
                key: "Mamouna".into(),
                planet_id: 40001200,
                resource_type_id: 42002000014,
                init_output: 29.200000762939453,
                richness_index: 2,
                richness_value: 1298,
            },
            21.83898263707198,
        )
    ];

    for resource in expected {
        let outpost = expected_outpost(&resource, &results);
        assert!(
            results.contains(&resource),
            "Expected {:#?} not in results {:#?}",
            outpost.0,
            outpost.1
        );
    }
}

fn expected_outpost(
    resource: &(CelestialResource, f64),
    results: &[(CelestialResource, f64)],
) -> (CelestialResource, Vec<(CelestialResource, f64)>) {
    let matching_results: Vec<(CelestialResource, f64)> = results
        .iter()
        .filter(|(result_resource, _)| {
            result_resource.key == resource.0.key &&
            result_resource.resource_type_id == resource.0.resource_type_id
        })
        .cloned() // Cloning the matching results to a new Vec
        .collect();

    (resource.0.clone(), matching_results)
}
