use clap::{Arg, ArgAction, ColorChoice, Command};

mod days;
mod error;
mod prelude;
mod utils;
use days::*;
use prelude::*;

fn main() -> Result<()> {
    let commands = generate_days()?;

    let args = vec![
        Arg::new("time taken")
            .short('t')
            .action(ArgAction::SetTrue)
            .conflicts_with("quiet"),
        Arg::new("quiet").short('q').action(ArgAction::SetTrue),
        Arg::new("file")
            .short('f')
            .value_name("file")
            .required(true),
    ];

    let matches = Command::new("adv")
        .color(ColorChoice::Always)
        .about("Advent of code 2022 cli utility")
        .args(args)
        .version("0.0.1")
        .arg_required_else_help(true)
        .subcommands(commands.clone())
        .subcommand_required(true)
        .get_matches();

    let subcommand = matches.subcommand().unwrap();

    for d in commands {
        if d.get_name() == subcommand.0 {
            d.run(matches.to_owned())?;
        }
    }
    Ok(())
}
