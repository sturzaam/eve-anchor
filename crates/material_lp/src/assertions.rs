use crate::data::{CelestialResource, celestial_resources_by_constellation, system_by_planet, get_celestial, get_constellation};
use crate::Material;

#[macro_export]
macro_rules! assert_arrays_leq {
    ($values:expr, $max_value:expr) => {
        let mut sum_by_planet: std::collections::HashMap<i64, f64> = std::collections::HashMap::new();
        let mut panic_messages = Vec::new();

        for &(resource, value) in $values.iter() {
            let planet_id = resource.planet_id;
            let sum = sum_by_planet.entry(planet_id).or_insert(0.0);
            *sum += value;

            if *sum > $max_value {
                let system = system_by_planet(planet_id)
                    .unwrap()
                    .en_name
                    .clone();
                let planet = get_celestial(planet_id)
                    .unwrap()
                    .celestial_index
                    .clone();
                let panic_message = format!(
                    "Sum of array's {} on {} {} exceeds {}",
                    sum,
                    system,
                    planet,
                    $max_value
                );
                panic_messages.push(panic_message);
            }
        }

        if !panic_messages.is_empty() {
            let joined_panic_message = panic_messages.join("\n");
            panic!("{}", joined_panic_message);
        }
    };
}

#[test]
fn format_assert_arrays_leq() {
    let solution: Vec<(&CelestialResource, f64)> = vec![
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000005,
            resource_type_id: 42001000032,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 22.),
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000005,
            resource_type_id: 42001000033,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 3.),
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000007,
            resource_type_id: 42001000032,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 21.),
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000007,
            resource_type_id: 42001000033,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 2.),
    ];
    
    let result = std::panic::catch_unwind(|| {
        assert_arrays_leq!(solution, 22.);
    });

    
    assert!(result.is_err(), "Expected a panic");
    let err = result.unwrap_err();
    let panic_message = err.downcast_ref::<String>().unwrap();
    assert!(
        panic_message.contains("Sum of array's 25 on Tanoo 2 exceeds 22"),
        "Unexpected panic message: {}",
        panic_message
    );
    assert!(
        panic_message.contains("Sum of array's 23 on Tanoo 3 exceeds 22"),
        "Unexpected panic message: {}",
        panic_message
    );
}

#[macro_export]
macro_rules! assert_planets_leq {
    ($values:expr, $max_value:expr) => {
        let mut count_by_constellation: std::collections::HashMap<i64, std::collections::HashSet<i64>> = std::collections::HashMap::new();
        let mut panic_messages = Vec::new();

        for &(resource, value) in $values.iter() {
            if value > 0.0 {
                let planet_id = resource.planet_id;
                let constellation_id = resource.constellation_id;
                let planet_ids = count_by_constellation.entry(constellation_id).or_insert(std::collections::HashSet::new());
                planet_ids.insert(planet_id);
            }
        }

        for (constellation_id, planet_ids) in count_by_constellation {
            let count = planet_ids.len();
            if count > $max_value {
                let constellation = get_constellation(constellation_id)
                    .unwrap()
                    .en_name
                    .clone();
                let panic_message = format!(
                    "Count of planets {} in {} exceeds {}",
                    count,
                    constellation,
                    $max_value
                );
                panic_messages.push(panic_message);
            }
        }

        if !panic_messages.is_empty() {
            let joined_panic_message = panic_messages.join("\n");
            panic!("{}", joined_panic_message);
        }
    };
}

#[test]
fn format_assert_planets_leq() {
    let solution: Vec<(&CelestialResource, f64)> = vec![
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000002,
            resource_type_id: 42001000032,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 22.),
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000005,
            resource_type_id: 42001000033,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 3.),
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000007,
            resource_type_id: 42001000032,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 21.),
        (&CelestialResource {
            constellation_id: 20000453,
            planet_id: 40000008,
            resource_type_id: 42001000033,
            init_output: 1.,
            richness_index: 1,
            richness_value: 1,
        }, 2.),
    ];
    
    let result = std::panic::catch_unwind(|| {
        assert_planets_leq!(solution, 3);
    });

    
    assert!(result.is_err(), "Expected a panic");
    let err = result.unwrap_err();
    let panic_message = err.downcast_ref::<String>().unwrap();
    assert!(
        panic_message.contains("Count of planets 4 in KUSW-P exceeds 3"),
        "Unexpected panic message: {}",
        panic_message
    );
}

#[macro_export]
macro_rules! assert_materials_in_constellations {
    ($materials:ident, $constellations:ident) => {
        let mut panic_messages = Vec::new();
        let mut found_in_any_constellation = false;

        for material in $materials.iter() {
            let mut material_found = false;

            for constellation in $constellations.iter() {
                let celestial_resources = celestial_resources_by_constellation(&constellation.0);
                let matching_celestial_resources: Vec<_> = celestial_resources
                .iter()
                .filter(|celestial_resource| celestial_resource.resource_type_id == material.resource_type_id)
                .collect();

                if !matching_celestial_resources.is_empty() {
                    material_found = true;
                    found_in_any_constellation = true;
                    break; // No need to check other constellations for this material
                }
            }

            if !material_found {
                let constellation_names: Vec<String> = $constellations.iter().map(|(name, _)| name.clone()).collect();
                let panic_message = format!(
                    "There is no known source of {} in {}",
                    material.name,
                    constellation_names.join(", ")
                );
                panic_messages.push(panic_message);
            }
        }

        if !found_in_any_constellation {
            let joined_panic_message = panic_messages.join("\n");
            panic!("{}", joined_panic_message);
        }
    };
}

#[test]
fn panic_when_materials_not_in_constellations() {
    let materials: Vec<Material> = vec![
        Material {
            resource_type_id: 42001000007,
            name: "Lucent Compound".into(),
            quantity: 0,
            valuation: 0.0
        }
    ];

    let constellations: Vec<(String, i32)> = vec![
        ("42-WDG".to_string(), 3),
    ];
    
    let result = std::panic::catch_unwind(|| {
        assert_materials_in_constellations!(materials, constellations);
    });

    
    assert!(result.is_err(), "Expected a panic");
    let err = result.unwrap_err();
    let panic_message = err.downcast_ref::<String>().unwrap();
    assert!(
        panic_message.contains("There is no known source of Lucent Compound in 42-WDG"),
        "Unexpected panic message: {}",
        panic_message
    );
}