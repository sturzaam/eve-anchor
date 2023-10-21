#[macro_export]
macro_rules! assert_materials_available {
    ($materials:expr, $outposts:expr) => {
        let mut panic_messages = Vec::new();
        let mut found_in_any_outpost = false;

        for material in $materials.iter() {
            let mut material_found = false;

            for outpost in $outposts.iter() {
                let celestial_resources = celestial_resources_by_outpost(outpost.clone());
                let matching_celestial_resources: Vec<_> = celestial_resources
                .iter()
                .filter(|celestial_resource| celestial_resource.resource_type_id == material.resource_type_id)
                .collect();

                if !matching_celestial_resources.is_empty() {
                    material_found = true;
                    break; // No need to check other outposts for this material
                }
            }

            if !material_found {
                let outpost_names: Vec<String> = $outposts.iter().map(|outpost| outpost.name.clone()).collect();
                let panic_message = format!(
                    "There is no known source of {} for {}",
                    material.name,
                    outpost_names.join(", ")
                );
                panic_messages.push(panic_message);
            }
        }

        if !found_in_any_outpost {
            let joined_panic_message = panic_messages.join("\n");
            panic!("{}", joined_panic_message);
        }
    };
}