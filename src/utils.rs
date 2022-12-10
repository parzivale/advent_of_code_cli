use std::time::Duration;

use crate::prelude::*;
use clap::{Arg, ArgMatches, Command};
use indicatif::ProgressBar;

#[derive(Clone)]
pub struct DayCommand<'a> {
    name: &'static str,
    about: &'static str,
    args: Vec<Arg>,
    subcommands: Vec<Command>,
    func: &'a dyn Fn(ArgMatches) -> Result<()>,
}

impl<'a> From<DayCommand<'a>> for Command {
    fn from(day: DayCommand<'a>) -> Self {
        let req = !day.subcommands.is_empty();
        Command::new(day.name)
            .about(day.about)
            .args(day.args)
            .subcommands(day.subcommands)
            .subcommand_required(req)
    }
}

impl<'a> DayCommand<'a> {
    pub fn run(self, args: ArgMatches) -> Result<()> {
        let func = self.func;
        let spin = ProgressBar::new_spinner();
        spin.enable_steady_tick(Duration::from_millis(100));
        spin.set_message("running command");
        func(args)?;
        ProgressBar::finish_and_clear(&spin);
        Ok(())
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}

pub struct DayCommandBuilder<'a> {
    name: Option<&'static str>,
    about: Option<&'static str>,
    args: Vec<Arg>,
    subcommands: Vec<Command>,
    func: &'a dyn Fn(ArgMatches) -> Result<()>,
}

impl<'a> DayCommandBuilder<'a> {
    pub fn new() -> Self {
        DayCommandBuilder {
            name: None,
            about: None,
            args: Vec::new(),
            subcommands: Vec::new(),
            func: &|_| Ok(()),
        }
    }

    pub fn name(&mut self, name: &'static str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn about(&mut self, about: &'static str) -> &mut Self {
        self.about = Some(about);
        self
    }

    pub fn arg(&mut self, arg: Arg) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn args(&mut self, args: &mut Vec<Arg>) -> &mut Self {
        self.args.append(args);
        self
    }

    pub fn subcommand(&mut self, subcommand: Command) -> &mut Self {
        self.subcommands.push(subcommand);
        self
    }

    pub fn subcommands(&mut self, subcommands: &mut Vec<Command>) -> &mut Self {
        self.subcommands.append(subcommands);
        self
    }

    pub fn func(&mut self, func: &'a dyn Fn(ArgMatches) -> Result<()>) -> &mut Self {
        self.func = func;
        self
    }

    pub fn build(&self) -> Result<DayCommand<'a>> {
        let name = self.name;

        if name.is_none() {
            return Err(Error::CommandBuilder("name wasn't defined".to_string()));
        }

        let name: &'static str = name.unwrap();

        let mut about = self.about;

        if about.is_none() {
            about = Some("");
        }

        let about: &'static str = about.unwrap();

        Ok(DayCommand {
            name,
            about,
            args: self.args.to_owned(),
            func: self.func,
            subcommands: self.subcommands.to_owned(),
        })
    }
}
