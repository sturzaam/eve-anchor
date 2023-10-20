mod data {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use lazy_static::lazy_static;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Celestial {
        pub constellation_id: i64,
        pub region_id: i64,
        pub solar_system_id: i64,
        pub celestial_index: i64,
    }
    
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
    pub struct Constellation {
            #[serde(default)]
            pub zh_name: Box<str>,
            #[serde(default)]
            pub en_name: Box<str>,
            #[serde(default)]
            pub de_name: Box<str>,
            #[serde(default)]
            pub fr_name: Box<str>,
            #[serde(default)]
            pub ja_name: Box<str>,
            #[serde(default)]
            pub por_name: Box<str>,
            #[serde(default)]
            pub ru_name: Box<str>,
            #[serde(default)]
            pub spa_name: Box<str>,
            #[serde(default)]
            pub zhcn_name: Box<str>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Item {
            #[serde(default)]
            pub zh_name: Box<str>,
            #[serde(default)]
            pub en_name: Box<str>,
            #[serde(default)]
            pub de_name: Box<str>,
            #[serde(default)]
            pub fr_name: Box<str>,
            #[serde(default)]
            pub ja_name: Box<str>,
            #[serde(default)]
            pub por_name: Box<str>,
            #[serde(default)]
            pub ru_name: Box<str>,
            #[serde(default)]
            pub spa_name: Box<str>,
            #[serde(default)]
            pub zhcn_name: Box<str>,
            #[serde(default)]
            pub kr_name: Box<str>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct System {
            #[serde(default)]
            pub zh_name: Box<str>,
            #[serde(default)]
            pub en_name: Box<str>,
            #[serde(default)]
            pub de_name: Box<str>,
            #[serde(default)]
            pub fr_name: Box<str>,
            #[serde(default)]
            pub ja_name: Box<str>,
            #[serde(default)]
            pub por_name: Box<str>,
            #[serde(default)]
            pub ru_name: Box<str>,
            #[serde(default)]
            pub spa_name: Box<str>,
            #[serde(default)]
            pub zhcn_name: Box<str>,
            #[serde(default)]
            pub constellation: Box<i64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
    pub struct Resource {
        pub init_output: f64,
        pub location_index: i64,
        pub resource_type_id: i64,
        pub richness_index: i64,
        pub richness_value: i64
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
    pub struct Planet {
        pub planet_id: i64,
        pub resource_info: HashMap<i64, Resource>
    }

    lazy_static! {
        pub static ref CELESTIALS: HashMap<i64, Celestial> = load_celestials().expect("Failed to load celestials");
        pub static ref CONSTELLATIONS: HashMap<i64, Constellation> = load_constellations().expect("Failed to load constellations.");
        pub static ref ITEMS: HashMap<i64, Item> = load_items().expect("Failed to load items.");
        pub static ref SYSTEMS: HashMap<i64, System> = load_systems().expect("Failed to load systems.");
        pub static ref PLANETS: HashMap<i64, Planet> = load_planet_exploit_resource().expect("Failed to load planets");
    }

    pub fn load_celestials() -> Result<HashMap<i64, Celestial>, Box<dyn std::error::Error>> {
        let out_dir = "./target".to_string();
        let file_path = std::path::Path::new(&out_dir).join("data/celestials.json");
    
        let celestial_data = std::fs::read_to_string(file_path)?;
        let celestials: HashMap<i64, Celestial> = serde_json::from_str(&celestial_data)?;
        Ok(celestials)
    }

    pub fn get_celestial(key: i64) -> Option<&'static Celestial> {
        CELESTIALS.get(&key)
    }

    pub fn load_items() -> Result<HashMap<i64, Item>, Box<dyn std::error::Error>> {
        let out_dir = "./target".to_string();
        let file_path = std::path::Path::new(&out_dir).join("data/all_items_info.json");
    
        let item_data = std::fs::read_to_string(file_path)?;
        let items: HashMap<i64, Item> = serde_json::from_str(&item_data)?;
        Ok(items)
    }

    pub fn get_item(key: i64) -> Option<&'static Item> {
        ITEMS.get(&key)
    }

    pub fn load_constellations() -> Result<HashMap<i64, Constellation>, Box<dyn std::error::Error>> {
        let out_dir = "./target".to_string();
        let file_path = std::path::Path::new(&out_dir).join("data/constellations_r.json");
        let constellation_data = std::fs::read_to_string(file_path)?;
        let constellations: HashMap<i64, Constellation> = serde_json::from_str(&constellation_data)?;
        Ok(constellations)
    }    

    pub fn get_constellation(key: i64) -> Option<&'static Constellation> {
        CONSTELLATIONS.get(&key)
    }

    pub fn load_planet_exploit_resource() -> Result<HashMap<i64, Planet>, Box<dyn std::error::Error>> {
        let out_dir = "./target".to_string();
        let file_path = std::path::Path::new(&out_dir).join("data/planet_exploit_resource.json");
    
        let planets_data = std::fs::read_to_string(file_path)?;
        let planets: HashMap<i64, Planet> = serde_json::from_str(&planets_data)?;
        Ok(planets)
    }

    pub fn get_system(key: i64) -> Option<&'static System> {
        SYSTEMS.get(&key)
    }

    pub fn load_systems() -> Result<HashMap<i64, System>, Box<dyn std::error::Error>> {
        let out_dir = "./target".to_string();
        let file_path = std::path::Path::new(&out_dir).join("data/systems_r.json");
        let system_data = std::fs::read_to_string(file_path)?;
        let systems: HashMap<i64, System> = serde_json::from_str(&system_data)?;
        Ok(systems)
    }

    pub fn find_system(name: &str) -> Option<&'static i64> {
        for (key, system) in SYSTEMS.iter() {
            if system.en_name.as_ref() == name {
                return Some(key);
            }
        }
        None
    }

    pub fn find_constellation_by_system(name: &str) -> Option<&'static i64> {
        let system_id = find_system(&name);
        for (_, celestial) in CELESTIALS.iter() {
            if celestial.solar_system_id == *system_id.unwrap() {
                return Some(&celestial.constellation_id);
            }
        }
        None
    }

    pub fn slice_celestials(constellation_id: i64) -> Result<HashMap<i64, Celestial>, Box<dyn std::error::Error>> {
        let sliced_celestials: HashMap<i64, Celestial> = CELESTIALS
            .iter()
            .filter(|(_, celestial)| celestial.constellation_id == constellation_id)
            .map(|(key, celestial)| (*key, celestial.clone()))
            .collect();
    
        Ok(sliced_celestials)
    }

    pub fn system_by_planet(key: i64) -> Option<&'static System> {
        return get_system(CELESTIALS.get(&key).unwrap().solar_system_id);
    }

    pub fn find_constellation(name: &str) -> Option<&'static i64>  {
        for (key, constellation) in CONSTELLATIONS.iter() {
            if constellation.en_name.as_ref() == name {
                return Some(key);
            }
        }
        None
    }

    pub fn find_item(name: &str) -> Option<&'static i64> {
        for (key, item) in ITEMS.iter() {
            if item.en_name.as_ref() == name {
                return Some(key);
            }
        }
        None
    }
}

pub use data::{get_item, get_celestial, get_constellation, get_system};
pub use data::{system_by_planet, find_item, find_constellation, find_system, find_constellation_by_system, slice_celestials};
pub use data::{Item, Constellation, Celestial, Resource, System, Planet};
pub use data::{CELESTIALS, CONSTELLATIONS, ITEMS, SYSTEMS, PLANETS};

