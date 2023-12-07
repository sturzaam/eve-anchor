#[cfg(test)]
mod tests {
    use material_lp::{
        manager::{Manager, Outpost},
        create_corporation,
        create_alliance,
        create_member,
        create_capsuleer,
        outposts_per_constellation
    };
    use material_lp::data::{get_system, get_constellation};
    

    fn setup_manager_with_outposts(manager: &mut Manager) {
        manager.add_outpost("A", "Tanoo", "Aaron", 12, 22).unwrap();
        manager.add_outpost("B", "Tanoo", "Benjamin", 12, 22).unwrap();
    }
    
    fn check_outpost_deletion(manager: &Manager, outpost_name: &str) {
        assert_eq!(manager.get_outposts().len(), 1);
        if manager.get_outposts().iter().any(|decoded| decoded.name == outpost_name) {
            panic!("Outpost still exists after deletion");
        }
    }
    
    #[test]
    fn test_capsuleer() {
        let capsuleer = create_capsuleer(
            "Alice",
        );
        assert_eq!(capsuleer.name, "Alice");
        assert_eq!(capsuleer.planetology, 0);
        assert_eq!(capsuleer.advanced_planetology, 5);
        assert_eq!(capsuleer.expert_planetology, 4);
    }

    #[test]
    fn test_new_outpost() {
        let capsuleer = create_capsuleer("Aaron");
        let outpost_result = Outpost::new("Outpost1".to_string(), capsuleer.clone(), "Tanoo".to_string(), 12, 22);
    
        match outpost_result {
            Ok(outpost) => {
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
                assert_eq!(outpost.capsuleer.planetology, 0);
                assert_eq!(outpost.capsuleer.advanced_planetology, 5);
                assert_eq!(outpost.capsuleer.expert_planetology, 4);
            }
            Err(err) => {
                panic!("Failed to create outpost: {}", err);
            }
        }
    }

    #[test]
    fn test_new_alliance_outpost() {
        let alliance = create_alliance("Alliance");
        let mut manager = Manager::Alliance(alliance);
        setup_manager_with_outposts(&mut manager);
    
        let expected = ("A".to_string(), 1);
        let outposts = outposts_per_constellation(&manager.get_outposts());
        assert!(outposts
            .as_ref()
            .expect("Failed to load outposts")
            .contains(&expected),
            "Expected {:?} is not found in outposts {:?}",
            expected,
            outposts
        );
    }
    
    #[test]
    fn test_delete_alliance_outpost() {
        let alliance = create_alliance("Alliance");
        let mut manager = Manager::Alliance(alliance);
        setup_manager_with_outposts(&mut manager);
    
        manager.delete_outpost("A").unwrap();
        let loaded_manager = manager.load_data().unwrap();
        check_outpost_deletion(&loaded_manager, "A");
    }

    #[test]
    fn test_new_corporation_outpost() {
        let corporation = create_corporation("Corporation", "Alliance");
        let mut manager = Manager::Corporation(corporation);
        setup_manager_with_outposts(&mut manager);
    
        let expected = ("A".to_string(), 1);
        let outposts = outposts_per_constellation(&manager.get_outposts());
        assert!(outposts
            .as_ref()
            .expect("Failed to load outposts")
            .contains(&expected),
            "Expected {:?} is not found in outposts {:?}",
            expected,
            outposts
        );
    }
    
    #[test]
    fn test_delete_corporation_outpost() {
        let corporation = create_corporation("Corporation", "Alliance");
        let mut manager = Manager::Corporation(corporation);
        setup_manager_with_outposts(&mut manager);
    
        manager.delete_outpost("A").unwrap();
        let loaded_manager = manager.load_data().unwrap();
        check_outpost_deletion(&loaded_manager, "A");
    }

    #[test]
    fn test_new_member_outpost() {
        let member = create_member("Member", "Corporation", "Alliance");
        let mut manager = Manager::Member(member);
        setup_manager_with_outposts(&mut manager);

        let expected = ("A".to_string(), 1);
        let outposts = outposts_per_constellation(&manager.get_outposts());
        assert!(outposts
            .as_ref()
            .expect("Failed to load outposts")
            .contains(&expected),
            "Expected {:?} is not found in outposts {:?}",
            expected,
            outposts
        );
    }

    #[test]
    fn test_delete_member_outpost() {
        let member = create_member("Member", "Corporation", "Alliance");
        let mut manager = Manager::Member(member);
        setup_manager_with_outposts(&mut manager);

        manager.delete_outpost("A").unwrap();
        let loaded_manager = manager.load_data().unwrap();
        check_outpost_deletion(&loaded_manager, "A");
    }
}