mod objective {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::HashMap;
    use arboard::Clipboard;
    use anyhow::Context;
    use crate::data::find_item;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Material {
        pub resource_type_id: i64,
        pub name: Box<str>,
        pub quantity: i64,
        pub valuation: f64,
    }


    pub fn push_material(line: &str, materials: &mut Vec<Material>) {
        let material_split: Vec<&str> = line.split("\t").collect();
        if material_split.len() >= 4 {
            materials.push(
                Material {
                    resource_type_id: *find_item(material_split[1].into()).unwrap(),
                    name: material_split[1].into(),
                    quantity: material_split[2].parse::<i64>().unwrap(),
                    valuation: material_split[3].trim().parse::<f64>().unwrap(),
                }
            );
        }
    }
    

    pub fn parse_decomposed_list() ->  Result<Vec::<Material>, anyhow::Error> {
        let expected_header = "ID\tNames\tQuantity\tValuation ";
        let mut materials = Vec::<Material>::new();
        let mut clipboard = Clipboard::new().context("Failed to create clipboard.")?;
        let text = clipboard.get_text().context("Failed to get clipboard text.")?;
        let mut lines = text.lines();

        if let Some(first_line) = lines.next() {
            if first_line != expected_header {
                return Err(anyhow::anyhow!("Invalid header line.").into());
            }
            for line in lines {
                push_material(line, &mut materials);
            }
        } else {
            return Err(anyhow::anyhow!("No header line.").into());
        }
        Ok(materials)
    }
}


pub use objective::push_material;
pub use objective::parse_decomposed_list;
pub use objective::Material;