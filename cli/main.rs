use clap::Parser;
use command::App;

mod command;

fn main() {
    let _matches = App::parse();
}
