use std::path::{Path, PathBuf};

use home::home_dir;

use crate::lockfile::{LockFile, LockFileReader};

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
    FileNotFound,
    LockFileError(crate::lockfile::LockFileError),
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
    fn load_or_create<T, E, O>(path: PathBuf, reader: O) -> Result<WorkspaceFile<T>, WorkspaceError>
    where
        O: FnOnce(String) -> Result<T, E>,
    {
        let content = std::fs::read_to_string(&path).map_err(|_| WorkspaceError::FileNotFound)?;
        let result = reader(content);
        match result {
            Ok(content) => return Ok(WorkspaceFile { path, content }),
            Err(_) => todo!(),
        }
    }

    pub fn default(options: WorkspaceOptions) -> Workspace {
        let lock_file_path = Path::new(&options.working_dir).join(LOCK_FILE);

        Workspace {
            options,
            lock_file: WorkspaceFile {
                path: lock_file_path,
                content: LockFile::new(),
            },
        }
    }

    pub fn load(options: WorkspaceOptions) -> Result<Workspace, WorkspaceError> {
        match std::fs::exists(&options.working_dir) {
            Ok(false) | Err(_) => return Err(WorkspaceError::NotFound),
            _ => (),
        };

        let lock_file = Workspace::load_or_create(
            Path::new(&options.working_dir).join(LOCK_FILE),
            LockFileReader::read,
        )
        .map_err(WorkspaceError::LockFileError)?;

        Ok(Workspace { options, lock_file })
    }
}
