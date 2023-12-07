use material_lp::manager::Outpost;
use material_lp::resource::{Material, celestial_resources_by_outpost};
use material_lp::{assert_materials_available, create_outpost};

#[test]
fn panic_when_materials_not_available() {
    let materials: Vec<Material> = vec![
        Material {
            resource_type_id: 42001000007,
            name: "Lucent Compound".into(),
            quantity: 0,
            valuation: 0.0
        },
        Material {
            resource_type_id: 42001000000,
            name: "Lustering Alloy".into(),
            quantity: 0,
            valuation: 0.0
        }
    ];

    let outposts: Vec<Outpost> = vec![
        create_outpost("Outpost1", "R97-CI", "Aaron"),
        create_outpost("Outpost2", "R97-CI", "Aaron"),
    ];
    
    let result = std::panic::catch_unwind(|| {
        assert_materials_available!(materials, outposts);
    });

    
    assert!(result.is_err(), "Expected a panic");
    let err = result.unwrap_err();
    let panic_message = err.downcast_ref::<String>().unwrap();
    assert!(
        panic_message.contains("There is no known source of Lucent Compound for Outpost1, Outpost2"),
        "Unexpected panic message: {}",
        panic_message
    );
}