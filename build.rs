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
    let mut output =
        File::create(podmanfile).expect(&format!("Could not open {} file", podmanfile));
    let contents =
        fs::read_to_string(podmantemplate).expect(&format!("Could not read {}", podmantemplate));
    output
        .write_all(contents.as_bytes())
        .expect("Error writing file");

    let c = Command::new("kubectl")
        .arg("create")
        .arg("configmap")
        .arg("--dry-run=client")
        .arg("rust-sql-exporter")
        .arg("--from-file=".to_owned() + queriesfile)
        .arg("-o=yaml")
        .output()
        .expect("failed to execute process");
    output.write_all(&c.stdout).expect("Error writing file");
}
