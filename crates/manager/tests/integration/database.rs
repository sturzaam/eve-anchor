// tests/database.rs

#[cfg(test)]
mod tests {

    use sea_orm::*;

    use manager::entities::*;
    use manager::entities::prelude::*;
    use manager::environment::EnvironmentManager;

    use crate::TEST_CAPSULEER_NAME;
    use crate::TEST_MEMBER_NAME;
    use crate::TEST_SKILL_NAME;
    use crate::DatabaseManager;

    #[tokio::test]
    async fn test_capsuleer() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");

        let member = member::ActiveModel {
            name: ActiveValue::Set(TEST_MEMBER_NAME.to_owned()),
            ..Default::default()
        };

        let saved_member = Member::insert(member.clone())
            .exec(&db)
            .await
            .expect("Failed to add member to database");

        let capsuleer = capsuleer::ActiveModel {
            name: ActiveValue::Set(TEST_CAPSULEER_NAME.to_owned()),
            member_id: ActiveValue::Set(saved_member.last_insert_id),
            ..Default::default()
        };

        let saved_capsuleer = Capsuleer::insert(capsuleer.clone())
            .exec(&db)
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

        let member = member::ActiveModel {
            name: ActiveValue::Set(TEST_MEMBER_NAME.to_owned()),
            ..Default::default()
        };

        let saved_member = Member::insert(member.clone())
            .exec(&db)
            .await
            .expect("Failed to add member to database");

        let capsuleer = capsuleer::ActiveModel {
            name: ActiveValue::Set(TEST_CAPSULEER_NAME.to_owned()),
            member_id: ActiveValue::Set(saved_member.last_insert_id),
            ..Default::default()
        };

        let saved_capsuleer = Capsuleer::insert(capsuleer.clone())
            .exec(&db)
            .await
            .expect("Failed to add capsuleer to database");

        let skill = skill::ActiveModel {
            name: ActiveValue::Set(TEST_SKILL_NAME.to_owned()),
            basic: ActiveValue::Set(0),
            advanced: ActiveValue::Set(0),
            expert: ActiveValue::Set(0),
            capsuleer_id: ActiveValue::Set(saved_capsuleer.last_insert_id),
            ..Default::default()
        };

        let saved_skill = Skill::insert(skill.clone())
            .exec(&db)
            .await
            .expect("Failed to add skill to database");
            
        let retrieved_skill: skill::Model = Skill::find_by_id(saved_skill.last_insert_id)
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_skill.name, TEST_SKILL_NAME);
        assert_eq!(retrieved_skill.basic, 0);
        assert_eq!(retrieved_skill.advanced, 0);
        assert_eq!(retrieved_skill.expert, 0);
    }
}