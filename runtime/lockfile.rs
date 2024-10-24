use std::collections::HashMap;

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

pub trait LockFileTrait {
    fn get_packages<'a>(&'a self) -> impl Iterator<Item = &Package> + 'a;
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
}

impl LockFileTrait for LockFile {
    fn get_packages<'a>(&'a self) -> impl Iterator<Item = &Package> + 'a {
        self.packages.values()
    }
}
