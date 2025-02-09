use glob::glob;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let mut output = File::create("queries.yaml").expect("Could not open queries.yaml file");

    for entry in glob("queries/*.yaml").expect("Failed to read queries directory") {
        match entry {
            Ok(path) => {
                let contents = fs::read_to_string(path).expect("Could not read file");
                output
                    .write_all(contents.as_bytes())
                    .expect("Error reading file");
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
