// tests/unit/capsuleer.rs

use crate::TEST_CAPSULEER_NAME;
use manager::entities::capsuleer;

#[test]
fn test_create_capsuleer() {
    let capsuleer = capsuleer::Model {
        id: 1,
        name: TEST_CAPSULEER_NAME.to_string(),
        active: true,
        member_id: 1,
    };
    
    assert_eq!(capsuleer.name, TEST_CAPSULEER_NAME.to_string());
    assert!(capsuleer.active);
}