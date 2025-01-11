mod resource {

    use serde::{Deserialize, Serialize};
    use manager::entities::outpost;


    use crate::data::{slice_celestials, get_constellation, find_constellation_by_system, PLANETS};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Material {
        pub resource_type_id: i64,
        pub name: Box<str>,
        pub quantity: i64,
        pub valuation: f64,
    }
        
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
    pub struct CelestialResource {
        pub key: String,
        pub planet_id: i64,
        pub resource_type_id: i64,
        pub init_output: f64,
        pub richness_index: i64,
        pub richness_value: i64
    }

    pub fn celestial_resources_by_outpost(outpost: outpost::Model) -> Vec<CelestialResource> {
        let mut celestial_resources: Vec<CelestialResource> = Vec::new();
        let constellation_id = find_constellation_by_system(&outpost.system).expect("Failed to find constellation by system");
        let celestials = slice_celestials(*constellation_id).expect("Failed to slice celestials");
        let outpost_name = outpost.name;

        for (_, planet) in PLANETS.iter().filter(|(key, _)| celestials.contains_key(*key)) {
            let planet_id = planet.planet_id;
            
            for (_, resource) in &planet.resource_info {
                celestial_resources.push(CelestialResource{
                    key: outpost_name.clone(),
                    planet_id,
                    resource_type_id: resource.resource_type_id,
                    init_output: resource.init_output,
                    richness_index: resource.richness_index,
                    richness_value: resource.richness_value,
                })
            }
        }
        celestial_resources
    }

    pub fn celestial_resources_by_constellation(constellation_id: i64) -> Vec<CelestialResource> {
        let mut celestial_resources: Vec<CelestialResource> = Vec::new();
        let celestials = slice_celestials(constellation_id).expect("Failed to slice celestials");

        for (_, planet) in PLANETS.iter().filter(|(key, _)| celestials.contains_key(*key)) {
            let planet_id = planet.planet_id;
            
            for (_, resource) in &planet.resource_info {
                celestial_resources.push(CelestialResource{
                    key: get_constellation(constellation_id).unwrap().en_name.to_string(),
                    planet_id,
                    resource_type_id: resource.resource_type_id,
                    init_output: resource.init_output,
                    richness_index: resource.richness_index,
                    richness_value: resource.richness_value,
                })
            }
        }
        celestial_resources
    }

}

pub use resource::{Material, CelestialResource, celestial_resources_by_outpost, celestial_resources_by_constellation};