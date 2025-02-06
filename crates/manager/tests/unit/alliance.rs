// tests/unit/alliance.rs

use crate::TEST_ALLIANCE_NAME;
use manager::entities::alliance;

#[test]
fn test_create_alliance() {
    let alliance = alliance::Model {
        id: 1,
        name: TEST_ALLIANCE_NAME.to_string(),
        active: true,
    };
    
    assert_eq!(alliance.name, TEST_ALLIANCE_NAME.to_string());
}