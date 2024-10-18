use clap::Parser;
use command::{Cli, Command};
use lockfile::LockFile;
use workspace::{Workspace, WorkspaceOptions};

mod command;
mod lockfile;
mod workspace;

fn main() {
    let cli = Cli::parse();
    let options = WorkspaceOptions::with_home();
    let workspace = match options {
        Ok(opts) => Workspace::new(opts),
        Err(_) => todo!("Implement Error"),
    };

    match cli.command {
        Command::Images => {
            println!("{:?}", workspace);
            LockFile::load("lock_file");
        }
        Command::Image(_) => {
            panic!("Not Implemented yet!")
        }
    }
}
