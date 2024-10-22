use std::{collections::HashMap, path::PathBuf};

use yaml_rust2::{Yaml, YamlLoader};

use crate::file::Reader;

#[derive(Debug)]
struct Package {
    version: String,
    integrity: String,
}

#[derive(Debug)]
pub struct LockFile {
    packages: HashMap<String, Package>,
}

pub struct LockFileReader {
    path: PathBuf,
}

pub enum LockFileError {
    PackagesNotFound,
    InvalidPackageName,
    MissingPackageProperty { property: String },
}

impl LockFileReader {
    pub fn new(path: PathBuf) -> LockFileReader {
        LockFileReader { path }
    }
}

impl Reader<LockFile, LockFileError> for LockFileReader {
    fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }

    fn read(&self, content: String) -> Result<LockFile, LockFileError> {
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

impl LockFile {
    pub fn new() -> LockFile {
        LockFile {
            packages: HashMap::new(),
        }
    }
}
