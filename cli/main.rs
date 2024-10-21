use clap::Parser;
use command::{Cli, Command};

use workspace::{Workspace, WorkspaceOptions};

mod command;
mod lockfile;
mod workspace;

fn main() {
    let cli = Cli::parse();
    let options = WorkspaceOptions::with_home();
    let workspace = match options {
        Ok(opts) => match Workspace::load(opts) {
            Ok(workspace) => workspace,
            Err(_) => Workspace::default(opts),
        },
        Err(_) => todo!("Implement Error"),
    };

    match cli.command {
        Command::Images => {
            println!("{:?}", workspace)
        }
        Command::Image(_) => {
            panic!("Not Implemented yet!")
        }
    }
}
