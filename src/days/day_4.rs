use clap::Arg;
use clap::ArgMatches;

use crate::prelude::*;
use crate::utils::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Task {
    start: i32,
    length: i32,
}

impl TryFrom<String> for Task {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        let s = s.replace("\r\n", "");
        let s_split = s.split('-').collect::<Vec<_>>();

        let length = s_split[1].parse::<i32>()? - s_split[0].parse::<i32>()?;

        Ok(Self {
            start: s_split[0].parse()?,
            length,
        })
    }
}

impl Task {
    pub fn contains(&self, task: &Task) -> bool {
        if task.start >= self.start && task.length < self.length {
            return true;
        }
        false
    }
}

pub fn day_4<'a>() -> Result<DayCommand<'a>> {
    DayCommandBuilder::new()
        .name("day_4")
        .arg(
            Arg::new("file")
                .alias("f")
                .help("the file with the data")
                .required(true),
        )
        .func(&day_4_func)
        .about("the solution to the day 4 advent of code problem")
        .build()
}

pub fn day_4_func(command: DayCommand, args: ArgMatches) -> Result<()> {
    let path: String = args.get_one::<String>("file").unwrap().to_owned();

    let f = File::open(path)?;

    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut threads = Vec::new();
    let mut len = reader.read_line(&mut line)?;
    let count = Arc::new(Mutex::new(0));

    while len != 0 {
        {
            let line = line.clone();
            let count = Arc::clone(&count);
            threads.push(thread::spawn(move || -> Result<()> {
                let line = line;
                let pair = line.split(',').collect::<Vec<_>>();

                let tasks1 = Task::try_from(pair[0].to_string())?;
                let tasks2 = Task::try_from(pair[1].to_string())?;

                if tasks1.contains(&tasks2) || tasks2.contains(&tasks1) {
                    println!("{:?}, {:?}", tasks1, tasks2);
                    *count.lock().unwrap() += 1;
                }

                Ok(())
            }));
        }

        line.clear();
        len = reader.read_line(&mut line)?;
    }

    for i in threads {
        i.join().unwrap()?;
    }

    println!("{:?} overlapping tasks", count.lock().unwrap());

    Ok(())
}
