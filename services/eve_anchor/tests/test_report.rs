#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use eve_anchor::{application_id, parse_requirements, load_outposts};
    use eve_anchor::report::{outpost_table, solution_table, material_table};
    use material_lp::resource::CelestialResource;
    use material_lp::create_outpost;

    #[test]
    fn pretty_material_table() {
        let requirements_value = "ID    Names    Quantity    Valuation \\n1    Lustering Alloy    4    10100069.2 \\n2    Smartfab Units    1    398.53";
        let materials = parse_requirements(requirements_value.to_string());
        let pretty_table = material_table(materials.unwrap());
        assert_eq!(pretty_table, "```
 Name             Quantity  Valuation 
 Lustering Alloy         4    10.100M 
 Smartfab Units          1     398.53 

```");
    }


    #[test]
    fn pretty_solution_table() {
        let outpost_name = "Outpost1".to_string();
        let solution: Vec<(CelestialResource, f64)> = vec![
            (CelestialResource {
                key: "Outpost1".to_string(),
                planet_id: 40000005,
                resource_type_id: 42001000032,
                init_output: 1.,
                richness_index: 1,
                richness_value: 1,
            }, 22.),
            (CelestialResource {
                key: "Outpost1".to_string(),
                planet_id: 40000005,
                resource_type_id: 42001000033,
                init_output: 1.,
                richness_index: 1,
                richness_value: 1,
            }, 3.),
            (CelestialResource {
                key: "Outpost1".to_string(),
                planet_id: 40000007,
                resource_type_id: 42001000032,
                init_output: 1.,
                richness_index: 1,
                richness_value: 1,
            }, 21.),
            (CelestialResource {
                key: "Outpost1".to_string(),
                planet_id: 40000007,
                resource_type_id: 42001000033,
                init_output: 1.,
                richness_index: 1,
                richness_value: 1,
            }, 2.),
        ];
        let pretty_table = solution_table(outpost_name, solution);
        assert_eq!(pretty_table, "```
Celestial Resource       Arrays
Tanoo 2   Silicate Glass 22
Tanoo 2   Smartfab Units 3
Tanoo 3   Silicate Glass 21
Tanoo 3   Smartfab Units 2

```");
    }

    #[test]
    fn pretty_outpost_table() {
        create_outpost(
            "Outpost1",
            "Tanoo",
            "Alice",
            "Corporation A",
            "Alliance A",
            "test"
        );
        let file_name = PathBuf::from(format!("./target/outpost/{}.bin", application_id()));
        println!("{}", file_name.display());
        let outposts = load_outposts(&file_name).unwrap();
        println!("{:?}", outposts);
        let pretty_table = outpost_table(outposts);
        assert_eq!(pretty_table, "```
Outpost  Capsuleer System
Outpost1 Alice     Tanoo

```");
    }
}