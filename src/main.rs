use clap::{Command};

use musicionist::cli::scales::{handle, scale_subcommand};

pub fn cli() -> Command {
    Command::new("musicionist")
        .about("A command line music theory tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(scale_subcommand())
}

fn main() {
    match cli().get_matches().subcommand() {
        Some(("scale", m)) => handle(m),
        _ => {}
    }
}
