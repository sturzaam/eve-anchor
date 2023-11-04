#[cfg(test)]
mod tests {
    use material_lp::resource::Material;
    use material_lp::create_outpost;
    use material_lp::objective::{
        map_objective,
        parse_decomposed_list,
        push_material,
        map_outpost,
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

    #[test]
    fn map_set_of_outposts() {
        let outposts = vec![
            create_outpost("Outpost1", "Tanoo", "Aaron", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost2", "Sooma", "Benjamin", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost3", "Futzchag", "Caroline", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost4", "Fovihi", "David", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost5", "Mohas", "Emily", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost6", "Dooz", "Fiona", "Corporation A", "Alliance A", "test"),
        ];
        let (available_outpost, available_planet, celestial_resources) = map_outpost(outposts);
        assert_eq!(available_outpost.len(), 6);
        assert_eq!(available_outpost.values().copied().sum::<f64>(), 1584.0);
        assert_eq!(available_planet.len(), 353);
        assert_eq!(available_planet.values().copied().sum::<f64>(), 7766.0);
        assert_eq!(celestial_resources.len(), 1030);

    }

    #[test]
    fn map_set_of_constellations() {
        let outposts = vec![
            create_outpost("Outpost1", "Tanoo", "Aaron", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost2", "Tanoo", "Benjamin", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost3", "Futzchag", "Caroline", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost4", "Futzchag", "David", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost5", "Mohas", "Emily", "Corporation A", "Alliance A", "test"),
            create_outpost("Outpost6", "Mohas", "Fiona", "Corporation A", "Alliance A", "test"),
        ];
        let (available_outpost, available_planet, celestial_resources) = map_constellation(outposts);
        assert_eq!(available_outpost.len(), 3);
        assert_eq!(available_outpost.values().copied().sum::<f64>(), 1584.0);
        assert_eq!(available_planet.len(), 174);
        assert_eq!(available_planet.values().copied().sum::<f64>(), 7656.0);
        assert_eq!(celestial_resources.len(), 990);
    }
}