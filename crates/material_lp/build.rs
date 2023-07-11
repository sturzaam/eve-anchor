use std::env;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use reqwest::Url;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
    let out_dir = PathBuf::from(out_dir);
    let auxilus_view = "https://auxilus.xyz/eve-echoes-data/download?view=/eve-echoes-data/";
    let data = vec![
        "all_items_info.json",
        "celestials.json",
        "constellations_r.json",
        "planet_exploit_resource.json",
        "systems_r.json",
    ];
    let data_dir = out_dir.join("data");

    fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    for filename in data {
        let file_url = format!("{}{}", auxilus_view, filename);
        let file_path = out_dir.join(&filename);
        let dest_path = data_dir.join(&filename);

        if !dest_path.exists() {
            let mut response = reqwest::blocking::get(Url::parse(&file_url).unwrap())
                .expect("Failed to send request");
            let mut file = File::create(&file_path).expect("Failed to create file");

            copy(&mut response, &mut file).expect("Failed to download file");
            fs::rename(&file_path, &dest_path).expect("Failed to move file to data directory");
        }
    }
}
