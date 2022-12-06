use crate::prelude::*;
use clap::{Arg, ArgMatches, Command};

#[derive(Clone)]
pub struct DayCommand<'a> {
    name: &'static str,
    about: &'static str,
    args: Vec<Arg>,
    func: &'a dyn Fn(Self, ArgMatches) -> Result<()>,
}

impl<'a> From<DayCommand<'a>> for Command {
    fn from(day: DayCommand<'a>) -> Self {
        Command::new(day.name).about(day.about).args(day.args)
    }
}

impl<'a> DayCommand<'a> {
    pub fn run(self, args: ArgMatches) -> Result<()> {
        let func = self.func;
        func(self, args)
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}

pub struct DayCommandBuilder<'a> {
    name: Option<&'static str>,
    about: Option<&'static str>,
    args: Vec<Arg>,
    func: &'a dyn Fn(DayCommand, ArgMatches) -> Result<()>,
}

impl<'a> DayCommandBuilder<'a> {
    pub fn new() -> Self {
        DayCommandBuilder {
            name: None,
            about: None,
            args: Vec::new(),
            func: &|_, _| Ok(()),
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

    pub fn func(&mut self, func: &'a dyn Fn(DayCommand, ArgMatches) -> Result<()>) -> &mut Self {
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
        })
    }
}
