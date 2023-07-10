use std::env;
use std::fs;
use std::path::PathBuf;
use glob::glob;

fn main() {
    // Get the output directory (OUT_DIR)
    let out_dir = env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");
    let out_dir = PathBuf::from(out_dir);

    // Specify the pattern for JSON files
    let pattern = "data/*.json";

    // Find all files matching the pattern
    let json_files: Vec<PathBuf> = glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();

    // Copy each JSON file to the output directory (OUT_DIR)
    for file in json_files {
        let destination = out_dir.join(file.file_name().unwrap());
        fs::copy(&file, &destination).expect("Failed to copy file");
    }
}
