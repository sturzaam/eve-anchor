#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use material_lp::{
        load_outposts,
        create_capsuleer,
        create_corporation,
        create_alliance,
    };
    use material_lp::structure::Outpost; 
    use material_lp::data::{get_system, get_constellation};


    #[test]
    fn test_alliance() {
        let alliance = create_alliance("Alliance A");
        assert_eq!(alliance.name, "Alliance A");
        assert_eq!(alliance.corporations.len(), 0);
    }
    
    #[test]
    fn test_corporation() {
        let corporation = create_corporation("Corporation A", "Alliance A");
        assert_eq!(corporation.name, "Corporation A");
        assert_eq!(corporation.alliance.name, "Alliance A");
        assert_eq!(corporation.outposts.len(), 0);
    }
    
    #[test]
    fn test_capsuleer() {
        let capsuleer = create_capsuleer(
            "Alice",
            "Corporation A",
            "Alliance A"
        );
        assert_eq!(capsuleer.name, "Alice");
        assert_eq!(capsuleer.corporation.name, "Corporation A");
        assert_eq!(capsuleer.corporation.alliance.name, "Alliance A");
        assert_eq!(capsuleer.planetology, 0);
        assert_eq!(capsuleer.advanced_planetology, 5);
        assert_eq!(capsuleer.expert_planetology, 4);
    }
    
    #[test]
    fn test_new_outpost() {
        let capsuleer = create_capsuleer("Aaron", "Corporation A", "Alliance A");
    
        // Attempt to create an outpost
        let outpost_result = Outpost::new("Outpost1".to_string(), capsuleer.clone(), "Tanoo".to_string(), "test".to_string());
    
        match outpost_result {
            Ok(outpost) => {
                // The operation was successful, you can perform your assertions here
                assert_eq!(outpost.name, "Outpost1");
                assert_eq!(outpost.system_id, 30000001);
                assert_eq!(
                    &*get_system(outpost.system_id).unwrap().zh_name,
                    "坦欧"
                );
                assert_eq!(outpost.constellation_id, 20000001);
                assert_eq!(
                    &*get_constellation(outpost.constellation_id).unwrap().zh_name,
                    "姗玛塔尔"
                );
                assert_eq!(outpost.capsuleer.name, "Aaron");
                assert_eq!(outpost.capsuleer.corporation.name, "Corporation A");
                assert_eq!(outpost.capsuleer.corporation.alliance.name, "Alliance A");
                assert_eq!(outpost.capsuleer.planetology, 0);
                assert_eq!(outpost.capsuleer.advanced_planetology, 5);
                assert_eq!(outpost.capsuleer.expert_planetology, 4);
                let expected = ("Outpost1".to_string(), 1);
                let constellations = load_outposts(&PathBuf::from("./target/outpost/test.bin".to_string()));
                assert!(constellations
                    .as_ref() // To avoid moving the Result
                    .expect("Failed to load constellations")
                    .contains(&expected),
                    "Expected {:?} is not found in constellations {:?}",
                    expected,
                    constellations
                );
            }
            Err(err) => {
                panic!("Failed to create outpost: {}", err);
            }
        }
    }
}