use std::{collections::HashMap, path::PathBuf};

use yaml_rust2::{Yaml, YamlLoader};

#[derive(Debug)]
struct Package {
    version: String,
    integrity: String,
}

#[derive(Debug)]
pub struct LockFile {
    packages: HashMap<String, Package>,
}

pub enum LockFileError {
    LockFileNotFound,
    PackagesNotFound,
    InvalidPackageName,
    MissingPackageProperty { property: String },
}

impl LockFile {
    pub fn default() -> LockFile {
        LockFile {
            packages: HashMap::new(),
        }
    }

    pub fn load(lock_file: &PathBuf) -> Result<LockFile, LockFileError> {
        fn get_package(node: &Yaml) -> Result<Package, LockFileError> {
            let version =
                node["version"]
                    .as_str()
                    .ok_or(LockFileError::MissingPackageProperty {
                        property: "version".to_string(),
                    })?;
            let integrity =
                node["integrity"]
                    .as_str()
                    .ok_or(LockFileError::MissingPackageProperty {
                        property: "integrity".to_string(),
                    })?;
            Ok(Package {
                version: version.to_string(),
                integrity: integrity.to_string(),
            })
        }

        let content =
            std::fs::read_to_string(lock_file).map_err(|_| LockFileError::LockFileNotFound)?;
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let root = docs[0]["packages"]
            .as_hash()
            .ok_or(LockFileError::PackagesNotFound)?;
        let packages: Result<HashMap<String, Package>, LockFileError> = root
            .iter()
            .map(
                |(name, values)| -> Result<(String, Package), LockFileError> {
                    let name = name.as_str().ok_or(LockFileError::InvalidPackageName)?;
                    let pkg = get_package(values)?;
                    Ok((name.to_string(), pkg))
                },
            )
            .collect();
        Ok(LockFile {
            packages: packages?,
        })
    }
}
