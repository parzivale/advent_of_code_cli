use crate::prelude::*;
use std::fmt::Display;
use std::iter::Sum;
use std::num::ParseIntError;

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
pub struct Calories {
    total: u32,
}

impl TryFrom<String> for Calories {
    type Error = ParseIntError;
    fn try_from(s: String) -> Result<Self, ParseIntError> {
        let s = s.trim();
        let splits = s.split(' ').collect::<Vec<_>>();
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

pub fn day_1() -> BoxResult<DayCommand> {
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

pub fn part_1(args: ArgMatches) -> BoxResult<()> {
    let f = FileReader::try_from(args)?;

    let mut max = Calories::new();
    let mut buf = String::new();

    for i in f {
        if i.is_empty() {
            let c = Calories::try_from(buf.clone());
            if let Ok(c) = c {
                if c > max {
                    max = c;
                }
            }
            buf.clear();
        }
        buf += &i;
        buf += " ";
    }

    println!("Maximum calories found was {}", max);

    Ok(())
}

pub fn part_2(args: ArgMatches) -> BoxResult<()> {
    let f = FileReader::try_from(args)?;

    let mut buf = String::new();

    let mut list = Vec::new();

    for i in f {
        if i.is_empty() {
            let c = Calories::try_from(buf.clone());
            if let Ok(c) = c {
                list.push(c);
                buf.clear()
                }
        }
        buf += &i;
        buf += " ";
    }

    list.sort_by(|a, b| b.cmp(a));

    println!(
        "Top 3 elves have {} calories",
        list[..3].iter().copied().sum::<Calories>()
    );
    Ok(())
}
