use std::collections::HashMap;

use yaml_rust2::{Yaml, YamlLoader};

use crate::file::{Builder, Reader};

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub integrity: String,
}

#[derive(Debug)]
pub struct LockFile {
    packages: HashMap<String, Package>,
}

pub struct LockFileReader;

pub struct LockFileDefault;

pub enum LockFileError {
    PackagesNotFound,
    InvalidPackageName,
    MissingPackageProperty { property: String },
}

impl Reader<LockFile, LockFileError> for LockFileReader {
    fn read(&self, content: String) -> Result<LockFile, LockFileError> {
        fn get_package((name, node): (&Yaml, &Yaml)) -> Result<Package, LockFileError> {
            let name = name.as_str().ok_or(LockFileError::InvalidPackageName)?;
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
                name: name.to_string(),
                version: version.to_string(),
                integrity: integrity.to_string(),
            })
        }

        let docs = YamlLoader::load_from_str(&content).unwrap();
        let root = docs[0]["packages"]
            .as_hash()
            .ok_or(LockFileError::PackagesNotFound)?;
        let packages = root
            .iter()
            .map(get_package)
            .collect::<Result<Vec<Package>, LockFileError>>()?;
        Ok(LockFile::new(packages))
    }
}

impl Builder<LockFile> for LockFileDefault {
    fn new(&self) -> LockFile {
        LockFile::new(Vec::new())
    }
}

impl LockFile {
    fn new(packages: Vec<Package>) -> LockFile {
        LockFile {
            packages: packages
                .into_iter()
                .map(|pkg| (pkg.name.to_string(), pkg))
                .collect(),
        }
    }
}
