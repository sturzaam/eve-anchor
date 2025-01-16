// tests/database.rs

#[cfg(test)]
mod tests {

    use sea_orm::*;

    use manager::*;
    use manager::entities::*;
    use manager::entities::prelude::*;
    use manager::environment::EnvironmentManager;

    use crate::DatabaseManager;
    use crate::TEST_ALLIANCE_NAME;
    use crate::TEST_CAPSULEER_NAME;
    use crate::TEST_CORPORATION_NAME;
    use crate::TEST_MEMBER_NAME;
    use crate::TEST_OUTPOST_NAME;
    use crate::TEST_PROBLEM_NAME;
    use crate::TEST_SKILL_NAME;
    use crate::TEST_SYSTEM_NAME;


    #[tokio::test]
    async fn test_alliance() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");

        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let retrieved_alliance = Alliance::find_by_name(TEST_ALLIANCE_NAME, &db)
            .await
            .unwrap()
            .unwrap();
    
        assert_eq!(retrieved_alliance.id, saved_alliance.last_insert_id);
        assert_eq!(retrieved_alliance.name, TEST_ALLIANCE_NAME);
        assert_eq!(retrieved_alliance.active, true);
    }

    #[tokio::test]
    async fn test_corporation() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let retrieved_corporation = Corporation::find_by_name(TEST_CORPORATION_NAME, &db)
            .await
            .unwrap()
            .unwrap();
    
        assert_eq!(retrieved_corporation.id, saved_corporation.last_insert_id);
        assert_eq!(retrieved_corporation.name, TEST_CORPORATION_NAME);
        assert_eq!(retrieved_corporation.active, true);
    }

    #[tokio::test]
    async fn test_capsuleer() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let saved_member = new_member(&db, TEST_MEMBER_NAME, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add member to database");

        let saved_capsuleer = new_capsuleer(&db, TEST_CAPSULEER_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add capsuleer to database");

        let retrieved_capsuleer = Capsuleer::find_by_name(TEST_CAPSULEER_NAME, &db)
            .await
            .unwrap()
            .unwrap();
    
        assert_eq!(retrieved_capsuleer.id, saved_capsuleer.last_insert_id);
        assert_eq!(retrieved_capsuleer.name, TEST_CAPSULEER_NAME);
        assert_eq!(retrieved_capsuleer.active, true);
    }


    #[tokio::test]
    async fn test_skill() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let saved_member = new_member(&db, TEST_MEMBER_NAME, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add member to database");

        let saved_capsuleer = new_capsuleer(&db, TEST_CAPSULEER_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add capsuleer to database");

        let saved_skill = new_skill(&db, TEST_SKILL_NAME, 5, 5, 4, saved_capsuleer.last_insert_id)
            .await
            .expect("Failed to add skill to database");
            
        let retrieved_skill: skill::Model = Skill::find_by_id(saved_skill.last_insert_id)
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_skill.name, TEST_SKILL_NAME);
        assert_eq!(retrieved_skill.basic, 5);
        assert_eq!(retrieved_skill.advanced, 5);
        assert_eq!(retrieved_skill.expert, 4);
    }

    #[tokio::test]
    async fn test_problem() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let saved_member = new_member(&db, TEST_MEMBER_NAME, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add member to database");

        let saved_problem = new_problem(&db, TEST_PROBLEM_NAME, vec![], saved_member.last_insert_id, saved_corporation.last_insert_id, None)
            .await
            .expect("Failed to add problem to database");
            
        let retrieved_problem: problem::Model = Problem::find_by_name(TEST_PROBLEM_NAME, &db)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_problem.id, saved_problem.last_insert_id);
        assert_eq!(retrieved_problem.name, TEST_PROBLEM_NAME);
        assert_eq!(retrieved_problem.active, true);
    }

    #[tokio::test]
    async fn test_outpost() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let saved_member = new_member(&db, TEST_MEMBER_NAME, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add member to database");

        let saved_capsuleer = new_capsuleer(&db, TEST_CAPSULEER_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add capsuleer to database");

        let saved_problem = new_problem(&db, TEST_PROBLEM_NAME, vec![], saved_member.last_insert_id, saved_corporation.last_insert_id, None)
            .await
            .expect("Failed to add problem to database");

        let saved_outpost = new_outpost(&db, TEST_OUTPOST_NAME, TEST_SYSTEM_NAME, 12, 26, saved_capsuleer.last_insert_id, Some(saved_problem.last_insert_id))
            .await
            .expect("Failed to add outpost to database");   

        let retrieved_outpost: outpost::Model = Outpost::find_by_name(TEST_OUTPOST_NAME, &db)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_outpost.id, saved_outpost.last_insert_id);
        assert_eq!(retrieved_outpost.name, TEST_OUTPOST_NAME);
        assert_eq!(retrieved_outpost.system, TEST_SYSTEM_NAME);
        assert_eq!(retrieved_outpost.planets, 12);
        assert_eq!(retrieved_outpost.arrays, 26);
    }

    #[tokio::test]
    async fn test_add_outpost_to_problem() {

        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let saved_member = new_member(&db, TEST_MEMBER_NAME, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add member to database");

        let saved_capsuleer = new_capsuleer(&db, TEST_CAPSULEER_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add capsuleer to database");

        let saved_outpost = new_outpost(&db, TEST_OUTPOST_NAME, TEST_SYSTEM_NAME, 12, 26, saved_capsuleer.last_insert_id, None)
            .await
            .expect("Failed to add outpost to database");
        
        let saved_problem = new_problem(&db, TEST_PROBLEM_NAME, vec![], saved_member.last_insert_id, saved_corporation.last_insert_id, None)
            .await
            .expect("Failed to add problem to database");

        let retrieved_outpost = Outpost::find_by_name(TEST_OUTPOST_NAME, &db)
            .await
            .unwrap()
            .unwrap();
        
        let mut active_outpost: outpost::ActiveModel = retrieved_outpost.into();
        active_outpost.problem_id = ActiveValue::Set(Some(saved_problem.last_insert_id));
        let _ = active_outpost
            .update(&db)
            .await
            .expect("Failed to update outpost");

        let retrieved_outpost: outpost::Model = Outpost::find_by_name(TEST_OUTPOST_NAME, &db)
            .await
            .unwrap()
            .unwrap();

        
        assert_eq!(retrieved_outpost.id, saved_outpost.last_insert_id);
        assert_eq!(retrieved_outpost.name, TEST_OUTPOST_NAME);
        assert_eq!(retrieved_outpost.system, TEST_SYSTEM_NAME);
        assert_eq!(retrieved_outpost.planets, 12);
        assert_eq!(retrieved_outpost.arrays, 26);
        assert_eq!(retrieved_outpost.problem_id, Some(saved_problem.last_insert_id));
    }

    #[tokio::test]
    async fn test_add_multiple_outpost_to_problem() {

        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");
            let saved_alliance = new_alliance(&db, TEST_ALLIANCE_NAME)
            .await
            .expect("Failed to add alliance to database");

        let saved_corporation = new_corporation(&db, TEST_CORPORATION_NAME, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add corporation to database");
        
        let saved_member = new_member(&db, TEST_MEMBER_NAME, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add member to database");

        let saved_capsuleer = new_capsuleer(&db, TEST_CAPSULEER_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id)
            .await
            .expect("Failed to add capsuleer to database");

        let saved_outpost = new_outpost(&db, TEST_OUTPOST_NAME, TEST_SYSTEM_NAME, 12, 26, saved_capsuleer.last_insert_id, None)
            .await
            .expect("Failed to add outpost to database");

        let saved_outpost2 = new_outpost(&db, "Test Outpost 2", TEST_SYSTEM_NAME, 12, 26, saved_capsuleer.last_insert_id, None)
            .await
            .expect("Failed to add outpost to database");

        let saved_problem = new_problem(&db, TEST_PROBLEM_NAME, vec![], saved_member.last_insert_id, saved_corporation.last_insert_id, None)
            .await
            .expect("Failed to add problem to database");

        let retrieved_outpost = Outpost::find_by_name(TEST_OUTPOST_NAME, &db)
            .await
            .unwrap()
            .unwrap();
        
        let mut active_outpost: outpost::ActiveModel = retrieved_outpost.into();

        active_outpost.problem_id = ActiveValue::Set(Some(saved_problem.last_insert_id));

        let _ = active_outpost
            .update(&db)
            .await
            .expect("Failed to update outpost");

        let retrieved_outpost_2 = Outpost::find_by_name("Test Outpost 2", &db)
            .await
            .unwrap()
            .unwrap();

        let mut active_outpost_2: outpost::ActiveModel = retrieved_outpost_2.into();

        active_outpost_2.problem_id = ActiveValue::Set(Some(saved_problem.last_insert_id));

        let _ = active_outpost_2
            .update(&db)
            .await
            .expect("Failed to update outpost");

        let retrieved_outposts_by_problem_name: Vec<(problem::Model, Option<outpost::Model>)> = Problem::find_outposts_by_name(TEST_PROBLEM_NAME, &db)
            .await
            .unwrap();

        assert_eq!(retrieved_outposts_by_problem_name[0].0.name, TEST_PROBLEM_NAME);
        assert_eq!(retrieved_outposts_by_problem_name[0].0.constraint, Vec::<u8>::new());
        assert_eq!(retrieved_outposts_by_problem_name.len(), 2);
        assert_eq!(retrieved_outposts_by_problem_name[0].1.clone().unwrap().name, TEST_OUTPOST_NAME);
        assert_eq!(retrieved_outposts_by_problem_name[1].1.clone().unwrap().name, "Test Outpost 2");
    }
}