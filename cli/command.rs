use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand, about = "Manage images")]
    Image(ImageCommands),

    #[command(about = "List installed images")]
    Images,
}

#[derive(Debug, Subcommand)]
pub enum ImageCommands {
    #[command(about = "Load an image from the specified path")]
    Load {
        #[arg(help = "Path that contains the image archive")]
        path: String,
    },
    #[command(about = "Remove image with specified name")]
    Rm {
        #[arg(help = "Name of the image")]
        name: String,
    },
}
