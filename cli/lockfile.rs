use std::{collections::HashMap, error::Error};

use yaml_rust2::{Yaml, YamlLoader};

#[derive(Debug)]
struct Package {
    version: String,
    integrity: String,
}

pub struct LockFile {
    packages: HashMap<String, Package>,
}

impl LockFile {
    pub fn load(lock_file: &str) -> Result<LockFile, Box<dyn Error>> {
        fn get_package(node: &Yaml) -> Result<Package, String> {
            let version = node["version"].as_str().ok_or("Missing package version")?;
            let integrity = node["integrity"]
                .as_str()
                .ok_or("Missing package integrity")?;
            Ok(Package {
                version: version.to_string(),
                integrity: integrity.to_string(),
            })
        }

        let content = std::fs::read_to_string(lock_file)?;
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let root = docs[0]["packages"].as_hash().ok_or("Missing packages")?;
        let packages: Result<HashMap<String, Package>, String> = root
            .iter()
            .map(|(name, values)| -> Result<(String, Package), String> {
                let name = name.as_str().ok_or("Invalid package name")?;
                let pkg = get_package(values)?;
                Ok((name.to_string(), pkg))
            })
            .collect();
        Ok(LockFile {
            packages: packages?,
        })
    }
}
