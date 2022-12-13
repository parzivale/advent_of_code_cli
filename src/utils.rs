use std::{
    ops::Deref,
    rc::Rc,
    time::{Duration, Instant},
};

use crate::prelude::*;
use clap::Command;
use indicatif::ProgressBar;

#[derive(Clone)]
pub struct DayCommand {
    name: &'static str,
    about: &'static str,
    parts: Vec<Part>,
}

#[derive(Clone)]
pub struct Part {
    name: &'static str,
    short_flag: char,
    func: Rc<dyn Fn(ArgMatches) -> Result<()>>,
    about: &'static str,
}

pub struct DayCommandBuilder {
    name: Option<&'static str>,
    about: Option<&'static str>,
    parts: Vec<Part>,
}

pub struct PartBuilder {
    name: Option<&'static str>,
    short_flag: Option<char>,
    func: Rc<dyn Fn(ArgMatches) -> Result<()>>,
    about: Option<&'static str>,
}

impl From<Part> for Command {
    fn from(part: Part) -> Self {
        Command::new(part.name)
            .about(part.about)
            .short_flag(part.short_flag)
    }
}

impl From<DayCommand> for Command {
    fn from(day: DayCommand) -> Self {
        Command::new(day.name)
            .about(day.about)
            .subcommands(day.parts)
            .subcommand_required(true)
    }
}

impl DayCommand {
    pub fn run(&self, args: ArgMatches) -> Result<()> {
        let (name, _) = args.subcommand().unwrap().1.subcommand().unwrap();

        let part = self
            .parts
            .iter()
            .find(|x| x.name == name)
            .ok_or(Error::CommandRunner("command part not found".to_string()))?;

        let ids: Vec<&str> = args.ids().into_iter().map(|x| x.as_str()).collect();

        let mut func: Box<dyn Fn(ArgMatches) -> Result<()>>;

        func = Box::new(Rc::deref(&part.func));

        for i in ids {
            if args.try_get_one::<bool>(i).is_err() {
                continue;
            }

            if args.get_flag(i) {
                func = match i {
                    "time taken" => Box::new(self.time_wrapper(func)),
                    _ => func,
                };
            }
        }

        let spin = ProgressBar::new_spinner();
        spin.enable_steady_tick(Duration::from_millis(100));
        spin.set_message("running command");
        func(args)?;
        spin.finish_and_clear();
        Ok(())
    }

    fn time_wrapper<F: Fn(ArgMatches) -> Result<()>>(
        &self,
        f: F,
    ) -> impl Fn(ArgMatches) -> Result<()> {
        move |args: ArgMatches| {
            let time = Instant::now();
            let res = f(args);
            let elapsed = time.elapsed().as_micros();
            println!("Time taken to execute command was {} microseconds", elapsed);
            res
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl PartBuilder {
    pub fn new() -> Self {
        PartBuilder {
            name: None,
            short_flag: None,
            func: Rc::new(|_| Ok(())),
            about: None,
        }
    }

    pub fn name(&mut self, name: &'static str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn short_flag(&mut self, flag: char) -> &mut Self {
        self.short_flag = Some(flag);
        self
    }

    pub fn about(&mut self, about: &'static str) -> &mut Self {
        self.about = Some(about);
        self
    }

    pub fn func(&mut self, func: impl Fn(ArgMatches) -> Result<()> + 'static) -> &mut Self {
        self.func = Rc::new(func);
        self
    }

    pub fn build(&self) -> Result<Part> {
        let name = self
            .name
            .ok_or(Error::CommandBuilder("name wasn't defined".to_string()))?;
        let about = self.about.unwrap_or_default();
        let short_flag = self.short_flag.unwrap_or(name.chars().last().unwrap());
        let func = Rc::clone(&self.func);

        Ok(Part {
            name,
            short_flag,
            func,
            about,
        })
    }
}

impl DayCommandBuilder {
    pub fn new() -> Self {
        DayCommandBuilder {
            name: None,
            about: None,
            parts: Vec::new(),
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

    pub fn parts(&mut self, parts: &mut Vec<Part>) -> &mut Self {
        self.parts.append(parts);
        self
    }

    pub fn build(&self) -> Result<DayCommand> {
        let name = self
            .name
            .ok_or(Error::CommandBuilder("name wasn't defined".to_string()))?;
        let about = self.about.unwrap_or_default();
        let parts = match self.parts.len() {
            0 => Err(Error::CommandBuilder("no parts were found".to_string())),
            _ => Ok(self.parts.to_owned()),
        }?;

        Ok(DayCommand { name, about, parts })
    }
}
