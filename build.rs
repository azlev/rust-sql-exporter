use glob::glob;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let filename = "queries.yaml";
    write_queries(filename);
}

fn write_queries(filename: &str) {
    let mut output = File::create(filename).expect("Could not open queries.yaml file");

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
