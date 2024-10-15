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
