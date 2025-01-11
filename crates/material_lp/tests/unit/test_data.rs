#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use material_lp::data::{get_item, get_celestial, get_constellation, get_system};
    use material_lp::data::{system_by_planet, find_item, find_constellation, find_system, find_constellation_by_system, slice_celestials};
    use material_lp::data::{Item, Constellation, Celestial, Resource, System, Planet};
    use material_lp::data::PLANETS;

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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
            constellation: Box::new(20000001),
        };
        let result = get_system(30000001);
        assert_eq!(Some(&system), result);
    }

    #[test]
    fn lookup_system_by_name() {
        let result = find_system("Tanoo");
        assert_eq!(result, Some(&30000001));
    }
    
    #[test]
    fn lookup_constellation_by_system() {
        let result = find_constellation_by_system("Tanoo");
        assert_eq!(result, Some(&20000001));
    }

    #[test]
    fn slice_celestial_by_constellation() {
        let constellation_id = find_constellation("KUSW-P").expect("Constellation not found");
        assert_eq!(*constellation_id, 20000453);
        let result = slice_celestials(*constellation_id).expect("Failed to slice celestials");
        assert_eq!(result.len(), 393);
    }
    
    #[test]
    fn lookup_system_by_planet() {
        let result = system_by_planet(40000002);
        assert_eq!(result.map(|system| system.en_name.as_ref()), Some("Tanoo"));
    }

    #[test]
    fn lookup_constellation_by_name() {
        let result = find_constellation("KUSW-P");
        assert_eq!(result, Some(&20000453));
    }

    #[test]
    fn lookup_item_by_name() {
        let result = find_item("Silicate Glass");
        assert_eq!(result, Some(&42001000032));
    }

    #[test]
    fn slice_resources_by_constellation() {
        let constellation_id = find_constellation("KUSW-P").expect("Constellation not found");
        assert_eq!(*constellation_id, 20000453);
        let celestials = slice_celestials(*constellation_id).expect("Failed to slice celestials");
        assert_eq!(celestials.len(), 393);
    }
}