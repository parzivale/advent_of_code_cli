use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::path::Path;

use clap::ArgMatches;

use crate::prelude::*;
use crate::utils::{DayCommand, DayCommandBuilder, PartBuilder};

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
pub struct Calories {
    total: u32,
}

impl TryFrom<String> for Calories {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        let splits = s.split("\r\n").collect::<Vec<_>>();
        let mut total = 0;

        for i in splits {
            total += i.parse::<u32>()?;
        }

        Ok(Self { total })
    }
}

impl Sum for Calories {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Calories::new();
        for i in iter {
            total.total += i.total;
        }
        total
    }
}

impl Display for Calories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.total)
    }
}

impl Calories {
    fn new() -> Self {
        Self { total: 0 }
    }
}

pub fn day_1() -> Result<DayCommand> {
    let mut parts = vec![
        PartBuilder::new()
            .name("part 1")
            .about("part 1 of the problem")
            .short_flag('1')
            .func(part_1)
            .build()?,
        PartBuilder::new()
            .name("part 2")
            .about("part 2 of the problem")
            .short_flag('2')
            .func(part_2)
            .build()?,
    ];

    DayCommandBuilder::new()
        .name("day_1")
        .parts(&mut parts)
        .about("the solution to the day 1 advent of code problem")
        .build()
}

pub fn part_1(args: ArgMatches) -> Result<()> {
    let path = args.get_one::<String>("file").unwrap();
    let path = Path::new(path);

    let f = File::open(path)?;
    let mut buf = String::new();
    let mut reader = BufReader::new(f);
    let mut eof = false;

    let mut max = Calories::new();

    while !eof {
        eof = reader.read_line(&mut buf)? == 0;
        if buf.contains("\r\n\r\n") {
            let s = &buf[..buf.len() - 4];
            let c = Calories::try_from(s.to_string())?;
            if c > max {
                max = c;
            }
            buf.clear();
        }
    }

    println!("Maximum calories found was {}", max);

    Ok(())
}

pub fn part_2(args: ArgMatches) -> Result<()> {
    let path = args.get_one::<String>("file").unwrap();
    let path = Path::new(path);

    let f = File::open(path)?;
    let mut buf = String::new();
    let mut reader = BufReader::new(f);
    let mut eof = false;

    let mut list = Vec::new();

    while !eof {
        eof = reader.read_line(&mut buf)? == 0;
        if buf.contains("\r\n\r\n") {
            let s = &buf[..buf.len() - 4];
            let c = Calories::try_from(s.to_string())?;
            list.push(c);
            buf.clear()
        }
    }

    list.sort_by(|a, b| b.cmp(a));

    println!(
        "Top 3 elves have {} calories",
        list[..3].iter().copied().sum::<Calories>()
    );
    Ok(())
}
