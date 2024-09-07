// tests/unit/member.rs

use crate::TEST_MEMBER_NAME;
use manager::entities::member;

#[test]
fn test_create_member() {
    let member = member::Model {
        id: 1,
        name: TEST_MEMBER_NAME.to_string(),
        active: true,
    };
    
    assert_eq!(member.name, TEST_MEMBER_NAME.to_string());
    assert!(member.active);
}