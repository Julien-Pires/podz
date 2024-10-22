use std::{collections::HashMap, path::PathBuf};

use yaml_rust2::{Yaml, YamlLoader};

use crate::file::Reader;

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

    fn new_empty(&self) -> LockFile {
        LockFile::new_empty()
    }

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

impl LockFile {
    pub fn new(packages: Vec<Package>) -> LockFile {
        LockFile {
            packages: packages
                .into_iter()
                .map(|pkg| (pkg.name.to_string(), pkg))
                .collect(),
        }
    }

    pub fn new_empty() -> LockFile {
        LockFile {
            packages: HashMap::new(),
        }
    }
}
