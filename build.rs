use glob::glob;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn main() {
    let podmantemplate: String = "podman/manifest.yaml.template".to_string();

    for dir in ["mssql", "postgres"] {
        write_queries(dir);
        let queriesfile = format!("queries_{dir}.yaml");
        let podmanfile = format!("podman/{dir}.yaml");
        write_podman(&podmanfile, &podmantemplate, &queriesfile);
    }
}

fn write_queries(dir: &str) {
    let mut output =
        File::create(format!("queries_{dir}.yaml")).expect("Could not open queries.yaml file");
    let pattern = format!("queries/{dir}/*.yaml");
    for entry in glob(&pattern).expect("Failed to read queries directory") {
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
        File::create(podmanfile).unwrap_or_else(|_| panic!("Could not open {} file", &podmanfile));
    let contents = fs::read_to_string(podmantemplate)
        .unwrap_or_else(|_| panic!("Could not read {}", &podmantemplate));
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
