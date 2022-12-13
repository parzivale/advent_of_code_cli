use clap::ArgMatches;

use crate::prelude::*;
use crate::utils::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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

    pub fn overlaps(&self, task: &Task) -> bool {
        self.start + self.length >= task.start && self.start <= task.start + task.length
    }
}

pub fn day_4() -> Result<DayCommand> {

    let mut parts = vec![
        PartBuilder::new()
            .name("part_1")
            .short_flag('1')
            .about("part 1 of the challenge")
            .func(part_1)
            .build()?,
        PartBuilder::new()
            .name("part_2")
            .short_flag('2')
            .about("part 2 of the challenge")
            .func(part_2)
            .build()?,
    ];

    DayCommandBuilder::new()
        .name("day_4")
        .parts(&mut parts)
        .about("the solution to the day 4 advent of code problem")
        .build()
}

pub fn part_1(args: ArgMatches) -> Result<()> {
    let path: String = args.get_one::<String>("file").unwrap().to_owned();

    let path = Path::new(&path);

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut not_eof: bool = reader.read_line(&mut line)? != 0;
    let mut count = 0;
    let mut total = 0;

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

    println!("Found {} contained tasks in {} pairs", count, total);

    Ok(())
}

pub fn part_2(args: ArgMatches) -> Result<()> {
    let path: String = args.get_one::<String>("file").unwrap().to_owned();

    let path = Path::new(&path);

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut not_eof: bool = reader.read_line(&mut line)? != 0;
    let mut count = 0;
    let mut total = 0;

    while not_eof {
        {
            total += 1;
            let line = line.clone();
            let pair = line.split(',').collect::<Vec<_>>();
            let tasks1 = Task::try_from(pair[0].to_string())?;
            let tasks2 = Task::try_from(pair[1].to_string())?;
            if tasks1.overlaps(&tasks2) {
                count += 1;
            }
        }

        line.clear();
        if reader.read_line(&mut line)? == 0 {
            not_eof = false;
        }
    }

    println!("Found {} overlapping tasks in {} pairs", count, total);

    Ok(())
}
