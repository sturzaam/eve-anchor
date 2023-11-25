use material_lp::{create_outpost, solve, solve_for_constellation};
use material_lp::objective::{
    map_outpost,
    map_objective,
    map_constellation,
    parse_decomposed_list
};
use material_lp::resource::{CelestialResource};
use material_lp::problem::{ResourceHarvestProblem};

#[test]
fn fuel_problem() {
    let outposts = vec![
    create_outpost("Outpost1", "Tanoo", "Aaron", "Corporation A", "Alliance A", "test"),
    create_outpost("Outpost2", "Sooma", "Benjamin", "Corporation A", "Alliance A", "test"),
    create_outpost("Outpost3", "Futzchag", "Caroline", "Corporation A", "Alliance A", "test"),
    create_outpost("Outpost4", "Fovihi", "David", "Corporation A", "Alliance A", "test"),
    create_outpost("Outpost5", "Mohas", "Emily", "Corporation A", "Alliance A", "test"),
    create_outpost("Outpost6", "Dooz", "Fiona", "Corporation A", "Alliance A", "test"),
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
    let (available_key, available_planet, celestial_resources) = map_outpost(outposts);
    let harvest = ResourceHarvestProblem::new(
        available_key,
        available_planet,
        minimum_output.clone(),
        value,
        7.,
    );
    assert_eq!(harvest.minimum_output.get(&42002000014), Some(&1395693.3076923075));

}

#[test]
fn using_outposts() {
    let outposts = vec![
        create_outpost("Outpost1", "Tanoo", "Aaron", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost2", "Sooma", "Benjamin", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost3", "Futzchag", "Caroline", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost4", "Fovihi", "David", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost5", "Mohas", "Emily", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost6", "Dooz", "Fiona", "Corporation A", "Alliance A", "test"),
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
    let results = solve(outposts, materials, 7.);
    assert_eq!(results.len(), 1030);

    let expected: Vec<(CelestialResource, f64)> = vec![
        (
            CelestialResource {
                key: "Outpost2".into(),
                planet_id: 40001002,
                resource_type_id: 42001000032,
                init_output: 10.399999618530272,
                richness_index: 3,
                richness_value: 1223,
            },
            22.
        ),(
            CelestialResource {
                key: "Outpost5".into(),
                planet_id: 40002134,
                resource_type_id: 42001000019,
                init_output: 18.84000015258789,
                richness_index: 2,
                richness_value: 1449,
            },
            22.
        ),
    ];

    for resource in expected {
        let outpost = expected_outpost(&resource, &results);
        assert!(
            results.contains(&resource),
            "Expected {:?} not in results {:?}",
            outpost.0,
            outpost.1
        );
    }
}


#[test]
fn using_constellation() {
    let outposts = vec![
        create_outpost("Outpost1", "Tanoo", "Aaron", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost2", "Tanoo", "Benjamin", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost3", "Futzchag", "Caroline", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost4", "Futzchag", "David", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost5", "Mohas", "Emily", "Corporation A", "Alliance A", "test"),
        create_outpost("Outpost6", "Mohas", "Fiona", "Corporation A", "Alliance A", "test"),
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
    let results = solve_for_constellation(outposts, materials, 7.);
    assert_eq!(results.len(), 990);

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
            44.0
        ),(
            CelestialResource {
                key: "Hevaka".into(),
                planet_id: 40002134,
                resource_type_id: 42001000019,
                init_output: 18.84000015258789,
                richness_index: 2,
                richness_value: 1449,
            },
            44.
        ),
    ];

    for resource in expected {
        let outpost = expected_outpost(&resource, &results);
        assert!(
            results.contains(&resource),
            "Expected {:?} not in results {:?}",
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
