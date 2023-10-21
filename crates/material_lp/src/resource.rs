mod resource {

    use serde::{Deserialize, Serialize};
    use crate::structure::Outpost;
    use crate::data::{slice_celestials, PLANETS};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Material {
        pub resource_type_id: i64,
        pub name: Box<str>,
        pub quantity: i64,
        pub valuation: f64,
    }
        
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
    pub struct CelestialResource {
        pub outpost_name: String,
        pub planet_id: i64,
        pub resource_type_id: i64,
        pub init_output: f64,
        pub richness_index: i64,
        pub richness_value: i64
    }

    pub fn celestial_resources_by_outpost(outpost: Outpost) -> Vec<CelestialResource> {
        let mut celestial_resources: Vec<CelestialResource> = Vec::new();
        let celestials = slice_celestials(outpost.constellation_id).expect("Failed to slice celestials");
        let outpost_name = outpost.name;

        for (_, planet) in PLANETS.iter().filter(|(key, _)| celestials.contains_key(*key)) {
            let planet_id = planet.planet_id;
            
            for (_, resource) in &planet.resource_info {
                celestial_resources.push(CelestialResource{
                    outpost_name: outpost_name.clone(),
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

pub use resource::{Material, CelestialResource, celestial_resources_by_outpost};