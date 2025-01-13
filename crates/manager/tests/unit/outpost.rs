// tests/unit/outpost.rs

use crate::TEST_OUTPOST_NAME;
use crate::TEST_SYSTEM_NAME;
use manager::entities::outpost;

#[test]
fn test_create_outpost() {
    let outpost = outpost::Model {
        id: 1,
        name: TEST_OUTPOST_NAME.to_string(),
        system: TEST_SYSTEM_NAME.to_string(),
        planets: 12,
        arrays: 22,
        capsuleer_id: 1,
        problem_id: None,
    };
    
    assert_eq!(outpost.name, TEST_OUTPOST_NAME.to_string());
    assert_eq!(outpost.system, TEST_SYSTEM_NAME.to_string());
    assert_eq!(outpost.planets, 12);
    assert_eq!(outpost.arrays, 22);
}