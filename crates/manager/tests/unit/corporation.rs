// tests/unit/corporation.rs

use crate::TEST_CORPORATION_NAME;
use manager::entities::corporation;

#[test]
fn test_create_corporation() {
    let corporation = corporation::Model {
        id: 1,
        name: TEST_CORPORATION_NAME.to_string(),
        active: true,
        alliance_id: 1,
    };
    
    assert_eq!(corporation.name, TEST_CORPORATION_NAME.to_string());
}