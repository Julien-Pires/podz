use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Manage images")]
    #[clap(subcommand)]
    Image(ImageCommands),

    #[command(about = "List installed images")]
    Images,
}

#[derive(Debug, Subcommand)]
enum ImageCommands {
    #[command(about = "Add a new image")]
    Add { name: String },
    #[command(about = "Remove specified image")]
    Remove { name: String },
}
