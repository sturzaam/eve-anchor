

#[cfg(test)]
mod tests {

    use manager::entities::*;
    use manager::environment::EnvironmentManager;
    use material_lp::resource::celestial_resources_by_outpost;
    use material_lp::assert_materials_available;

    use crate::DatabaseManager;
    use material_lp::resource::Material;
    use material_lp::create_outpost;



    #[tokio::test]
    async fn panic_when_materials_not_available() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");
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

        let outposts: Vec<outpost::Model> = vec![
            create_outpost(&db, "Outpost1", "R97-CI", "Aaron").await,
            create_outpost(&db, "Outpost2", "R97-CI", "Aaron").await,
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
}