use clap::{ColorChoice, Command};

mod days;
mod error;
mod prelude;
mod utils;
use days::*;
use prelude::*;

fn main() -> Result<()> {
    let commands = generate_days()?;

    let matches = Command::new("adv")
        .color(ColorChoice::Always)
        .about("Advent of code 2022 cli utility")
        .version("0.0.1")
        .arg_required_else_help(true)
        .subcommands(commands.clone())
        .get_matches();

    let subcommand = matches.subcommand().unwrap();

    for d in commands {
        if d.get_name() == subcommand.0 {
            d.run(subcommand.1.to_owned())?
        }
    }
    Ok(())
}
