use material_lp::{map_objective, parse_decomposed_list};
use material_lp::{assert_planets_leq};
use material_lp::data::{get_constellation};
use material_lp::solver::{ResourceHarvestProblem};
use material_lp::data::{CelestialResource};
use arboard::Clipboard;
use good_lp::solvers::Solution;

#[test]
fn using_solution_table() {
    let constellations: Vec<(String, i32)> = vec![
        ("FY6-NK".to_string(), 3),
        ("R2-BT6".to_string(), 3),
        ("E-ILCH".to_string(), 3),
        ("IZ-FBT".to_string(), 3),
        ("42-WDG".to_string(), 7),
        ("7ZRW-G".to_string(), 5),
    ];
    let mut clipboard = Clipboard::new().unwrap();
    let the_string = "ID	Names	Quantity	Valuation 
9	Reactive Gas	3218648	629728481.2 
10	Industrial Fibers	297200	356574616.0 
11	Coolant	1638448	995275237.6 
12	Construction Blocks	2347224	896123178.72 
13	Silicate Glass	1114176	1126810755.84 
14	Polyaramids	1143896	117741215.28 
15	Noble Gas	1106272	401797990.4 
16	Supertensile Plastics	660272	338422413.6 
17	Nanites	407040	589630003.2 
18	Smartfab Units	830512	347403169.6 
19	Condensates	913696	316778403.2 
";
    let _ = clipboard.set_text(the_string);
    let materials = parse_decomposed_list();
    let available_outposts = constellations.iter().map(|(_, value)| value).sum();
    let (available_constellation, available_planet, minimum_output, celestial_resources, value) 
        = map_objective(materials.unwrap(), constellations.clone());
    let mut harvest = ResourceHarvestProblem::new(
        available_constellation,
        available_planet,
        minimum_output,
        value,
        available_outposts,
        7.,
    );
    let variables: Vec<_> = celestial_resources
        .clone()
        .into_iter()
        .map(|r| harvest.add_resource(r))
        .collect();
    let solution = harvest.best_production();
    let resource_quantities: Vec<_> = variables.iter().map(|&v| solution.value(v)).collect();
    let celestial_resource_values: Vec<(&CelestialResource, f64)> = celestial_resources
        .iter()
        .zip(resource_quantities.iter().cloned())
        .collect();
    
    let result = std::panic::catch_unwind(|| {
        assert_planets_leq!(celestial_resource_values, 12);
    });
    assert!(result.is_err(), "Expected a panic");

    let err = result.unwrap_err();
    let panic_message = err.downcast_ref::<String>().unwrap();
    assert!(
        panic_message.contains("Count of planets 13 in FY6-NK exceeds 12"),
        "Unexpected panic message: {}",
        panic_message
    );
    assert!(
        panic_message.contains("Count of planets 13 in R2-BT6 exceeds 12"),
        "Unexpected panic message: {}",
        panic_message
    );
}
