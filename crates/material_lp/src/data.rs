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
    
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
    pub struct CelestialResource {
        pub constellation_id: i64,
        pub planet_id: i64,
        pub resource_type_id: i64,
        pub init_output: f64,
        pub richness_index: i64,
        pub richness_value: i64
    }

    lazy_static! {
        static ref CELESTIALS: HashMap<i64, Celestial> = load_celestials().expect("Failed to load celestials");
        static ref CONSTELLATIONS: HashMap<i64, Constellation> = load_constellations().expect("Failed to load constellations.");
        static ref ITEMS: HashMap<i64, Item> = load_items().expect("Failed to load items.");
        static ref SYSTEMS: HashMap<i64, System> = load_systems().expect("Failed to load systems.");
        static ref PLANETS: HashMap<i64, Planet> = load_planet_exploit_resource().expect("Failed to load planets");
    }

    #[cfg(test)]
    fn load_data_celestials_from_json() {
        let celestial = Celestial {
            constellation_id: 20000001,
            region_id: 10000001,
            solar_system_id: 30000001,
            celestial_index: 1,
        };
        let result = get_celestial(40000002);
        assert_eq!(Some(&celestial), result);
    }

    pub fn load_celestials() -> Result<HashMap<i64, Celestial>, Box<dyn std::error::Error>> {
        let out_dir = std::env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
        let file_path = std::path::Path::new(&out_dir).join("data/celestials.json");
    
        let celestial_data = std::fs::read_to_string(file_path)?;
        let celestials: HashMap<i64, Celestial> = serde_json::from_str(&celestial_data)?;
        Ok(celestials)
    }

    pub fn get_celestial(key: i64) -> Option<&'static Celestial> {
        CELESTIALS.get(&key)
    }

    #[cfg(test)]
    fn load_data_items_from_json() {
        let item = Item {
            zh_name: "硅结构铸材".into(),
            en_name: "Silicate Glass".into(),
            de_name: "Silicate Glass".into(),
            fr_name: "Verre de silice".into(),
            ja_name: "シリコンガラス".into(),
            por_name: "Vidro de silicato".into(),
            ru_name: "Silicate Glass".into(),
            spa_name: "Vidrio de silicato".into(),
            zhcn_name: "硅结构铸材".into(),
            kr_name: "실리케이트 글라스".into(),
        };
        let result = get_item(42001000032);
        assert_eq!(Some(&item), result);
    }

    pub fn load_items() -> Result<HashMap<i64, Item>, Box<dyn std::error::Error>> {
        let out_dir = std::env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
        let file_path = std::path::Path::new(&out_dir).join("data/all_items_info.json");
    
        let item_data = std::fs::read_to_string(file_path)?;
        let items: HashMap<i64, Item> = serde_json::from_str(&item_data)?;
        Ok(items)
    }

    pub fn get_item(key: i64) -> Option<&'static Item> {
        ITEMS.get(&key)
    }

    #[cfg(test)]
    fn load_data_constellations_from_json() {
        let constellation = Constellation {
            zh_name: "KUSW-P".into(),
            en_name: "KUSW-P".into(),
            de_name: "KUSW-P".into(),
            fr_name: "KUSW-P".into(),
            ja_name: "KUSW-P".into(),
            por_name: "KUSW-P".into(),
            ru_name: "KUSW-P".into(),
            spa_name: "KUSW-P".into(),
            zhcn_name: "KUSW-P".into(),
        };
        let result = get_constellation(20000453);
        assert_eq!(Some(&constellation), result);
    }

    pub fn load_constellations() -> Result<HashMap<i64, Constellation>, Box<dyn std::error::Error>> {
        let out_dir = std::env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
        let file_path = std::path::Path::new(&out_dir).join("data/constellations_r.json");
    
        let constellation_data = std::fs::read_to_string(file_path)?;
        let constellations: HashMap<i64, Constellation> = serde_json::from_str(&constellation_data)?;
        Ok(constellations)
    }    

    pub fn get_constellation(key: i64) -> Option<&'static Constellation> {
        CONSTELLATIONS.get(&key)
    }

    #[cfg(test)]
    fn load_data_resources_from_json() {
        let resource_42001000020 = Resource {
            init_output: 3.549999952316284,
            location_index: 1,
            resource_type_id: 42001000020,
            richness_index: 3,
            richness_value: 1123
        };
        let resource_42001000029 = Resource {
            init_output: 9.899999618530272,
            location_index: 0,
            resource_type_id: 42001000029,
            richness_index: 3,
            richness_value: 1238
        };
        let resource_42002000012 = Resource {
            init_output: 154.1999969482422,
            location_index: 5,
            resource_type_id: 42002000012,
            richness_index: 2,
            richness_value: 1285
        };
        let resource_info: HashMap<i64, Resource> = HashMap::from([
            (42002000012, resource_42002000012),
            (42001000029, resource_42001000029),
            (42001000020, resource_42001000020),
        ]);
        let planet = Planet {
            planet_id: 40000002,
            resource_info: resource_info
        };
        let result = PLANETS.get(&40000002);
        assert_eq!(Some(&planet), result);
    }

    pub fn load_planet_exploit_resource() -> Result<HashMap<i64, Planet>, Box<dyn std::error::Error>> {
        let out_dir = std::env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
        let file_path = std::path::Path::new(&out_dir).join("data/planet_exploit_resource.json");
    
        let planets_data = std::fs::read_to_string(file_path)?;
        let planets: HashMap<i64, Planet> = serde_json::from_str(&planets_data)?;
        Ok(planets)
    }

    #[cfg(test)]
    fn load_data_systems_from_json() {
        let system = System {
            zh_name: "坦欧".into(),
            en_name: "Tanoo".into(),
            de_name: "Tanoo".into(),
            fr_name: "Tanoo".into(),
            ja_name: "タヌー".into(),
            por_name: "Tanoo".into(),
            ru_name: "Tanoo".into(),
            spa_name: "Tanoo".into(),
            zhcn_name: "坦欧".into(),
        };
        let result = get_system(30000001);
        assert_eq!(Some(&system), result);
    }

    pub fn get_system(key: i64) -> Option<&'static System> {
        SYSTEMS.get(&key)
    }

    pub fn load_systems() -> Result<HashMap<i64, System>, Box<dyn std::error::Error>> {
        let out_dir = std::env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
        let file_path = std::path::Path::new(&out_dir).join("data/systems_r.json");
    
        let system_data = std::fs::read_to_string(file_path)?;
        let systems: HashMap<i64, System> = serde_json::from_str(&system_data)?;
        Ok(systems)
    }    

    #[cfg(test)]
    fn slice_celestial_by_constellation() {
        let constellation_id = find_constellation("KUSW-P").expect("Constellation not found");
        assert_eq!(*constellation_id, 20000453);
        let result = slice_celestials(*constellation_id).expect("Failed to slice celestials");
        assert_eq!(result.len(), 393);
    }

    pub fn slice_celestials(constellation_id: i64) -> Result<HashMap<i64, Celestial>, Box<dyn std::error::Error>> {
        let sliced_celestials: HashMap<i64, Celestial> = CELESTIALS
            .iter()
            .filter(|(_, celestial)| celestial.constellation_id == constellation_id)
            .map(|(key, celestial)| (*key, celestial.clone()))
            .collect();
    
        Ok(sliced_celestials)
    }
    
    #[cfg(test)]
    fn lookup_system_by_planet() {
        let result = system_by_planet(40000002);
        assert_eq!(result.map(|system| system.en_name.as_ref()), Some("Tanoo"));
    }

    pub fn system_by_planet(key: i64) -> Option<&'static System> {
        return get_system(CELESTIALS.get(&key).unwrap().solar_system_id);
    }

    #[cfg(test)]
    fn lookup_constellation_by_name() {
        let result = find_constellation("KUSW-P");
        assert_eq!(result, Some(&20000453));
    }

    pub fn find_constellation(name: &str) -> Option<&'static i64>  {
        for (key, constellation) in CONSTELLATIONS.iter() {
            if constellation.en_name.as_ref() == name {
                return Some(key);
            }
        }
        None
    }

    #[cfg(test)]
    fn lookup_item_by_name() {
        let result = find_item("Silicate Glass");
        assert_eq!(result, Some(&42001000032));
    }

    pub fn find_item(name: &str) -> Option<&'static i64> {
        for (key, item) in ITEMS.iter() {
            if item.en_name.as_ref() == name {
                return Some(key);
            }
        }
        None
    }

    #[cfg(test)]
    fn slice_resources_by_constellation() {
        let constellation_id = find_constellation("KUSW-P").expect("Constellation not found");
        assert_eq!(*constellation_id, 20000453);
        let celestials = slice_celestials(*constellation_id).expect("Failed to slice celestials");
        assert_eq!(celestials.len(), 393);
        let planet_resources = available_planets_by_constellation("KUSW-P", 22.).expect("Failed to slice planet resources");
        assert_eq!(planet_resources.len(), 63);
    }
    
    pub fn available_planets_by_constellation(name: &str, number: f64) -> Result<HashMap<i64, f64>, Box<dyn std::error::Error>> {
        let constellation_id = find_constellation(name).expect(&format!("Constellation '{}' not found", name));
        let celestials = slice_celestials(*constellation_id).expect("Failed to slice celestials");
        let available_planets: HashMap<i64, f64> = PLANETS
            .iter()
            .filter(|(key, _)| celestials.contains_key(*key))
            .map(|(key, _)| (*key, number))
            .collect();

        Ok(available_planets)
    }

    #[cfg(test)]
    fn slice_celestial_resources_by_constellation_name() {
        let celestial_resources: Vec<CelestialResource> = celestial_resources_by_constellation("KUSW-P");
        assert_eq!(celestial_resources.len(), 289);
        let resource = celestial_resources
            .iter()
            .find(|resource| resource.resource_type_id == 42001000004 && resource.planet_id == 40197096)
            .cloned()
            .unwrap();
        assert_eq!(
            resource.init_output,
            20.8700008392334
        );
    }

    pub fn celestial_resources_by_constellation(name: &str) -> Vec<CelestialResource> {
        let mut celestial_resources: Vec<CelestialResource> = Vec::new();
        let constellation_id = find_constellation(name).expect("Constellation not found");
        let celestials = slice_celestials(*constellation_id).expect("Failed to slice celestials");
    
        for (_, planet) in PLANETS.iter().filter(|(key, _)| celestials.contains_key(*key)) {
            let planet_id = planet.planet_id;
            
            for (_, resource) in &planet.resource_info {
                celestial_resources.push(CelestialResource{
                    constellation_id: *constellation_id,
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

pub use data::{get_item, get_celestial, get_constellation};
pub use data::{system_by_planet, find_item, find_constellation, slice_celestials};
pub use data::{celestial_resources_by_constellation, available_planets_by_constellation};
pub use data::{Item, Constellation, Celestial, Resource, CelestialResource};
