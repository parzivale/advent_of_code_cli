
use clap::Arg;
use clap::ArgMatches;

use crate::prelude::*;
use crate::utils::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::{Duration,Instant};
use indicatif::ProgressBar;

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
        task.start >= self.start
            && task.length <= self.length
            && task.start + task.length <= self.start + self.length
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

pub fn day_4_func(args: ArgMatches) -> Result<()> {
    let time = Instant::now();
    let path: String = args.get_one::<String>("file").unwrap().to_owned();

    let f = File::open(path)?;

    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut not_eof:bool = reader.read_line(&mut line)? != 0;
    let mut count = 0;
    let mut total = 0;
    let spin = ProgressBar::new_spinner();
    spin.enable_steady_tick(Duration::from_millis(100));
    spin.set_message("running command");
    while not_eof {
        {
            total += 1;
            let line = line.clone();
                let pair = line.split(',').collect::<Vec<_>>();
                let tasks1 = Task::try_from(pair[0].to_string())?;
                let tasks2 = Task::try_from(pair[1].to_string())?;
                if tasks1.contains(&tasks2) || tasks2.contains(&tasks1) {
                    count += 1;
                }
                }

        line.clear();
        if reader.read_line(&mut line)? == 0 {
            not_eof = false;
        }
    }

    let time_taken = time.elapsed();
    ProgressBar::finish_and_clear(&spin);
    println!("Finished in {} milliseconds", time_taken.as_millis());
    println!("Found {} overlapping tasks in {} pairs", count, total);

    Ok(())
}
