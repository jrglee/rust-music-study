use clap::{Command};

use musicionist::cli::scales::{handle, handle_interactive, scale_subcommand};

pub fn cli() -> Command {
    Command::new("musicionist")
        .about("A command line music theory tool")
        .subcommand(scale_subcommand())
}

fn main() -> anyhow::Result<()> {
    match cli().get_matches().subcommand() {
        Some(("scale", m)) => handle(m)?,
        _ => {
            let choice = inquire::Select::new("What do you want to explore?", vec!["scale"])
                .prompt()?;
            match choice {
                "scale" => handle_interactive()?,
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}
