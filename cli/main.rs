use clap::{arg, command, Command};

fn main() {
    let _matches = command!()
        .subcommand(
            Command::new("images")
                .subcommand(Command::new("list").about("List installed images"))
                .subcommand(
                    Command::new("add")
                        .about("Add a new image")
                        .arg(arg!([image] "Image name")),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove specified image")
                        .arg(arg!([image] "Image name")),
                ),
        )
        .get_matches();
}
