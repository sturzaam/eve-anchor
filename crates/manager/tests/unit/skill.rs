// tests/unit/skill.rs

use crate::TEST_SKILL_NAME;
use manager::entities::skill;

#[test]
fn test_create_skill() {
    let skill = skill::Model {
        id: 1,
        name: TEST_SKILL_NAME.to_string(),
        basic: 0,
        advanced: 5,
        expert: 4,
        capsuleer_id: 1,
    };
    
    assert_eq!(skill.name, TEST_SKILL_NAME);
    assert_eq!(skill.basic, 0);
    assert_eq!(skill.advanced, 5);
    assert_eq!(skill.expert, 4);
}