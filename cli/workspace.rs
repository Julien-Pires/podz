use std::path::{Path, PathBuf};

use home::home_dir;

use crate::{
    file::{Builder, FileReader},
    lockfile::{CliLockFile, CliLockFileReader},
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
    pub lockfile: WorkspaceFile<CliLockFile>,
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
    fn load_or_create<TFile, TError, TReader: FileReader<TFile, TError> + Builder<TFile>>(
        path: PathBuf,
        reader: TReader,
    ) -> Result<WorkspaceFile<TFile>, WorkspaceError> {
        let result = std::fs::read(&path);
        let file = match result {
            Ok(content) => reader
                .read(content)
                .map_err(|_| WorkspaceError::FileReadError(path.to_path_buf()))?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => reader.new(),
            Err(_) => return Err(WorkspaceError::FileError(path.to_path_buf())),
        };
        Ok(WorkspaceFile {
            path,
            content: file,
        })
    }

    pub fn new(options: WorkspaceOptions) -> Result<Workspace, WorkspaceError> {
        match std::fs::exists(&options.working_dir) {
            Ok(false) | Err(_) => return Err(WorkspaceError::NotFound),
            _ => (),
        };

        let lockfile = Workspace::load_or_create(
            Path::new(&options.working_dir).join(LOCK_FILE),
            CliLockFileReader,
        )?;
        Ok(Workspace { options, lockfile })
    }
}
