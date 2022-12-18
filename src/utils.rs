use std::{
    fmt::Display,
    ops::Deref,
    rc::Rc,
    time::{Duration, Instant}, error::Error,
};

use crate::{
    error::{DayCommandBuilderError, DayCommandError, FromArgs},
    prelude::*,
};
use clap::Command;
use indicatif::ProgressBar;

pub struct FileReader {
    reader: BufReader<File>,
    buf: String,
}

impl TryFrom<ArgMatches> for FileReader {
    type Error = FromArgs;
    fn try_from(args: ArgMatches) -> Result<Self, FromArgs> {
        let path = args
            .get_one::<String>("file")
            .ok_or(FromArgs::FieldNotFound("path".to_string()))?;
        let path = Path::new(path);

        let f = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(f),
            buf: String::new(),
        })
    }
}

impl Iterator for FileReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        let res = self.reader.read_line(&mut self.buf);

        if res.is_err() {
            return None;
        }

        if res.unwrap() == 0 {
            return None;
        }

        Some(self.buf.clone().trim().to_string())
    }
}

pub struct CommandResponse<T> {
    value: T,
    pretty_print: String,
    info: String,
}

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
    func: Rc<dyn Fn(ArgMatches) -> BoxResult<()>>,
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
    func: Rc<dyn Fn(ArgMatches) -> Result<(), Box<dyn Error>>>,
    about: Option<&'static str>,
}

impl<T:Display> Display for CommandResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.pretty_print, self.value)
    }
}

impl<T> CommandResponse<T> {}

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
    pub fn run(&self, args: ArgMatches) -> BoxResult<()> {
        let (name, _) = args.subcommand().unwrap().1.subcommand().unwrap();

        let part = self
            .parts
            .iter()
            .find(|x| x.name == name)
            .ok_or(DayCommandError::CommandPartNotFound(name.to_string()))?;

        let ids: Vec<&str> = args.ids().into_iter().map(|x| x.as_str()).collect();

        let mut func: Box<dyn Fn(ArgMatches) -> BoxResult<()>>;

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

    fn time_wrapper<F: Fn(ArgMatches) -> BoxResult<()>>(
        &self,
        f: F,
    ) -> impl Fn(ArgMatches) -> BoxResult<()> {
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

    pub fn func(&mut self, func: impl Fn(ArgMatches) -> BoxResult<()> + 'static) -> &mut Self {
        self.func = Rc::new(func);
        self
    }

    pub fn build(&self) -> BoxResult<Part> {
        let name = self.name.ok_or(DayCommandBuilderError::NameNotFound)?;
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

    pub fn build(&self) -> BoxResult<DayCommand> {
        let name = self.name.ok_or(DayCommandBuilderError::NameNotFound)?;
        let about = self.about.unwrap_or_default();
        let parts = match self.parts.len() {
            0 => Err(DayCommandBuilderError::PartsNotFound),
            _ => Ok(self.parts.to_owned()),
        }?;

        Ok(DayCommand { name, about, parts })
    }
}
