use podz_runtime::lockfile::{LockFile, LockFileTrait, Package};
use yaml_rust2::{Yaml, YamlLoader};

use crate::file::FileReader;

pub struct CliLockFileReader;

pub enum CliLockFileError {
    PackagesNotFound,
    InvalidPackageName,
    MissingPackageProperty { property: String },
}

pub struct CliLockFile {
    lockfile: LockFile,
}

impl FileReader<LockFile, CliLockFileError> for CliLockFileReader {
    fn read(&self, content: [u8]) -> Result<LockFile, CliLockFileError> {
        fn get_package((name, node): (&Yaml, &Yaml)) -> Result<Package, CliLockFileError> {
            let name = name.as_str().ok_or(CliLockFileError::InvalidPackageName)?;
            let version =
                node["version"]
                    .as_str()
                    .ok_or(CliLockFileError::MissingPackageProperty {
                        property: "version".to_string(),
                    })?;
            let integrity =
                node["integrity"]
                    .as_str()
                    .ok_or(CliLockFileError::MissingPackageProperty {
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
            .ok_or(CliLockFileError::PackagesNotFound)?;
        let packages = root
            .iter()
            .map(get_package)
            .collect::<Result<Vec<Package>, CliLockFileError>>()?;
        Ok(LockFile::new(packages))
    }
}

impl LockFileTrait for CliLockFile {
    fn get_packages<'a>(&'a self) -> impl Iterator<Item = &Package> + 'a {
        self.lockfile.get_packages()
    }
}
