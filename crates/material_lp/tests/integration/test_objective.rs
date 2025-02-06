

#[cfg(test)]
mod tests {

    use manager::environment::EnvironmentManager;

    use crate::DatabaseManager;
    use material_lp::resource::Material;
    use material_lp::create_outpost;
    use material_lp::objective::{
        map_objective,
        parse_decomposed_list,
        push_material,
        map_constellation
    };
    

    #[test]
    fn parse_invalid_header() {
        let materials = parse_decomposed_list("");
        assert!(materials.is_err());
        assert!(materials.unwrap_err().to_string().contains("No header line."));
    }
    
    #[test]
    fn parse_a_decomposed_list() {
        let material = Material {
            resource_type_id: 42001000000,
            name: "Lustering Alloy".into(),
            quantity: 4,
            valuation: 10100069.2
        };
        let materials = parse_decomposed_list("ID	Names	Quantity	Valuation 
        1	Lustering Alloy	4	10100069.2 
        2	Liquid Ozone	1	171.78 
        ");
        assert!(materials.unwrap().contains(&material));
    }

    #[test]
    fn map_an_objective() {
        let materials = parse_decomposed_list("ID	Names	Quantity	Valuation 
        1	Lustering Alloy	1	167.09 
        2	Sheen Compound	2	400.00 
        ");
        let (minimum_output, value) = map_objective(materials.unwrap());
        assert_eq!(minimum_output.get(&42001000000), Some(&1.));
        assert_eq!(value.lustering_allow, 167.09);
        assert_eq!(minimum_output.get(&42001000001), Some(&2.));
        assert_eq!(value.sheen_compound, 200.00);
    }
    
    #[test]
    fn push_a_material() {
        let mut result = Vec::<Material>::new();
        let materials: Vec<Material> = vec![
            Material {
                resource_type_id: 42001000000,
                name: "Lustering Alloy".into(),
                quantity: 4,
                valuation: 10100069.2
            }
        ];
        push_material("1	Lustering Alloy	4	10100069.2 ", &mut result);
        assert_eq!(result, materials);
    }

    #[tokio::test]
    async fn map_set_of_constellations() {
        let config = EnvironmentManager::load_config("test")
            .await
            .expect("Failed to load configuration");
        let db = DatabaseManager::revision(&config)
            .await
            .expect("Failed to connect to database");
        let outposts = vec![
            create_outpost(&db, "Outpost1", "Tanoo", "Aaron").await,
            create_outpost(&db, "Outpost2", "Tanoo", "Benjamin").await,
            create_outpost(&db, "Outpost3", "Futzchag", "Caroline").await,
            create_outpost(&db, "Outpost4", "Futzchag", "David").await,
            create_outpost(&db, "Outpost5", "Mohas", "Emily").await,
            create_outpost(&db, "Outpost6", "Mohas", "Fiona").await,
        ];
        let (available_outpost, available_planet, celestial_resources) = map_constellation(outposts);
        assert_eq!(available_outpost.len(), 3);
        assert_eq!(available_outpost.values().copied().sum::<i32>(), 1872);
        assert_eq!(available_planet.len(), 174);
        assert_eq!(available_planet.values().copied().sum::<i32>(), 9048);
        assert_eq!(celestial_resources.len(), 990);
    }
}