use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = PathBuf::from("target");

    Command::new("tar")
        .args(&["-xzvf", "data.tar.gz", "-C", target.to_str().unwrap()])
        .status()
        .expect("Failed to extract tar.gz file");

    for entry in fs::read_dir("target/data").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("gz") {
            Command::new("gunzip")
                .arg(path.to_str().unwrap())
                .status()
                .expect("Failed to unzip file");
        }
    }
}
