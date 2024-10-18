use std::path::{Path, PathBuf};

use home::home_dir;

static WORKSPACE_DIR: &str = ".podz";
static LOCK_FILE: &str = "podz-lock.yaml";

#[derive(Debug)]
pub struct Workspace {
    pub working_dir: PathBuf,
    pub lock_file: PathBuf,
}

pub struct WorkspaceOptions {
    working_dir: PathBuf,
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
    pub fn new(options: WorkspaceOptions) -> Workspace {
        Workspace {
            working_dir: options.working_dir.to_path_buf(),
            lock_file: Path::new(&options.working_dir).join(LOCK_FILE),
        }
    }
}
