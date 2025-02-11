use glob::glob;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn main() {
    let queriesfile = "queries.yaml";
    let podmantemplate = "podman/manifest.yaml.template";
    let podmanfile = "podman/manifest.yaml";

    write_queries(queriesfile);
    write_podman(podmanfile, podmantemplate, queriesfile);
}

fn write_queries(filename: &str) {
    let mut output = File::create(filename).expect("Could not open queries.yaml file");

    for entry in glob("queries/*.yaml").expect("Failed to read queries directory") {
        match entry {
            Ok(path) => {
                let contents = fs::read_to_string(path).expect("Could not read file");
                output
                    .write_all(contents.as_bytes())
                    .expect("Error writing file");
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}

fn write_podman(podmanfile: &str, podmantemplate: &str, queriesfile: &str) {
    Command::new("sh")
        .arg("-c")
        .arg("cat podman/manifest.yaml.template <(kubectl create configmap --dry-run=client rust-sql-exporter --from-file=queries.yaml -o yaml) > podman/manifest.yaml")
        .output()
        .expect("failed to execute process");
}
