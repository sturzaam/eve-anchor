// tests/unit/problem.rs

use crate::TEST_PROBLEM_NAME;
use manager::entities::problem;

#[test]
fn test_create_problem() {
    let problem = problem::Model {
        id: 1,
        name: TEST_PROBLEM_NAME.to_string(),
        active: true,
        member_id: 1,
        corporation_id: 1,
        alliance_id: None,
    };
    
    assert_eq!(problem.name, TEST_PROBLEM_NAME);
    assert!(problem.active);
}