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

        let saved_problem = new_problem(&db, TEST_PROBLEM_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id, saved_alliance.last_insert_id)
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

        let saved_problem = new_problem(&db, TEST_PROBLEM_NAME, saved_member.last_insert_id, saved_corporation.last_insert_id, saved_alliance.last_insert_id)
            .await
            .expect("Failed to add problem to database");

        let saved_outpost = new_outpost(&db, TEST_OUTPOST_NAME, TEST_SYSTEM_NAME, 12, 26, saved_capsuleer.last_insert_id, saved_problem.last_insert_id)
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
}