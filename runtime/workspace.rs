use std::path::{Path, PathBuf};

use home::home_dir;

use crate::{
    file::Reader,
    lockfile::{LockFile, LockFileReader},
};

static WORKSPACE_DIR: &str = ".podz";
static LOCK_FILE: &str = "podz-lock.yaml";

#[derive(Debug)]
pub struct WorkspaceOptions {
    working_dir: PathBuf,
}

#[derive(Debug)]
pub struct WorkspaceFile<T> {
    path: PathBuf,
    content: T,
}

#[derive(Debug)]
pub struct Workspace {
    pub options: WorkspaceOptions,
    pub lock_file: WorkspaceFile<LockFile>,
}

pub enum WorkspaceError {
    NotFound,
    FileError(PathBuf),
    FileReadError(PathBuf),
}

impl WorkspaceOptions {
    pub fn with_home() -> Result<WorkspaceOptions, String> {
        match home_dir() {
            Some(cwd) => {
                return Ok(WorkspaceOptions {
                    working_dir: Path::new(&cwd).join(WORKSPACE_DIR),
                })
            }
            None => return Err("ERRRR".to_string()),
        };
    }
}

impl Workspace {
    fn load_or_create<TFile, TError>(
        reader: impl Reader<TFile, TError>,
    ) -> Result<WorkspaceFile<TFile>, WorkspaceError> {
        let result = std::fs::read_to_string(reader.path());
        let file = match result {
            Ok(content) => reader
                .read(content)
                .map_err(|_| WorkspaceError::FileReadError(reader.path()))?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => reader.new_empty(),
            Err(_) => return Err(WorkspaceError::FileError(reader.path())),
        };

        Ok(WorkspaceFile {
            path: reader.path(),
            content: file,
        })
    }

    pub fn load(options: WorkspaceOptions) -> Result<Workspace, WorkspaceError> {
        match std::fs::exists(&options.working_dir) {
            Ok(false) | Err(_) => return Err(WorkspaceError::NotFound),
            _ => (),
        };

        let lock_file = Workspace::load_or_create(LockFileReader::new(
            Path::new(&options.working_dir).join(LOCK_FILE),
        ))?;

        Ok(Workspace { options, lock_file })
    }
}
