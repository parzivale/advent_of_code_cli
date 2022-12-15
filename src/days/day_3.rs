use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Rucksack {
    comp1: String,
    comp2: String,
    priority: i32,
}
#[derive(Debug)]
pub struct Group {
    rucksacks: Vec<Rucksack>,
    priority: i32
}

impl From<&Vec<Rucksack>> for Group {
    fn from(value: &Vec<Rucksack>) -> Self {
        let value = &value[..3];
        let mut a = Group {
            rucksacks: value.to_vec(),
            priority:0
        };

        a.priority = a.priority();
        a
    }
}

impl Group {
    pub fn priority(&self) -> i32 {
        let mut storage = self
            .rucksacks
            .iter()
            .map(|x| x.comp1.to_owned() + &x.comp2)
            .collect::<Vec<_>>();
        let last = storage.pop().unwrap();
        let mut chars = Vec::new();
        for i in last.chars() {
            if storage.iter().all(|x| x.contains(i)) {
                chars.push(i);
            }
        }
        chars.sort();
        chars.dedup();
        chars
            .iter()
            .map(|x| {
                if x.is_lowercase() {
                    x.to_digit(36).unwrap() as i32 - 9
                } else {
                    x.to_digit(36).unwrap() as i32 + 17
                }
            })
            .sum::<i32>()
    }
}

impl TryFrom<String> for Rucksack {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let value = value.trim().to_string();
        if value.is_empty() {
            return Err(Error::CharParse("Rucksack".to_string()));
        }

        if value.len() % 2 != 0 {
            return Err(Error::CharParse("Rucksack".to_string()));
        }

        let (comp1, comp2) = value.split_at(value.len() / 2);
        let comp1 = comp1.to_string();
        let comp2 = comp2.to_string();

        let mut matches = comp1
            .chars()
            .filter(|x| comp2.contains(*x))
            .collect::<Vec<_>>();

        matches.sort();
        matches.dedup();

        let priority: i32 = matches
            .iter()
            .map(|x| {
                if x.is_lowercase() {
                    x.to_digit(36).unwrap() as i32 - 9
                } else {
                    x.to_digit(36).unwrap() as i32 + 17
                }
            })
            .sum();

        Ok(Self {
            comp1,
            comp2,
            priority,
        })
    }
}

pub fn day_3() -> Result<DayCommand> {
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
        .name("day_3")
        .parts(&mut parts)
        .about("the solution to the day 3 advent of code problem")
        .build()
}

pub fn part_1(args: ArgMatches) -> Result<()> {
    let f = FileReader::try_from(args)?;
    let mut rucksacks = Vec::new();

    for i in f {
        rucksacks.push(Rucksack::try_from(i.to_owned())?);
    }

    println!(
        "The sum of priorities is {}",
        rucksacks.iter().map(|x| x.priority).sum::<i32>()
    );
    Ok(())
}

pub fn part_2(args: ArgMatches) -> Result<()> {
    let f = FileReader::try_from(args)?;
    let mut groups = Vec::new();
    let mut buf = Vec::new();

    for (n, i) in f.enumerate() {
        buf.push(Rucksack::try_from(i)?);
        if (n + 1) % 3 == 0{
            groups.push(Group::from(&buf));
            buf.clear();
        }
    }



    println!(
        "priorty of all the groups is {}",
        groups.iter().map(|x| x.priority).sum::<i32>()
    );
    Ok(())
}
