use std::cmp::Ordering;

use crate::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    pub fn corrected_read(c: char, elf_move: &Rps) -> Result<Self> {
        let t = match c {
            'X' => match elf_move {
                Rps::Rock => Rps::Scissors,
                Rps::Paper => Rps::Rock,
                Rps::Scissors => Rps::Paper,
            },
            'Y' => elf_move.clone(),
            'Z' => match elf_move {
                Rps::Rock => Rps::Paper,
                Rps::Paper => Rps::Scissors,
                Rps::Scissors => Rps::Rock,
            },
            _ => {
                return Err(Error::CharParse(
                    "char not found for rock paper scissors".to_string(),
                ))
            }
        };
        Ok(t)
    }
}

impl TryFrom<char> for Rps {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Rps::Rock),
            'B' => Ok(Rps::Paper),
            'C' => Ok(Rps::Scissors),
            'X' => Ok(Rps::Rock),
            'Y' => Ok(Rps::Paper),
            'Z' => Ok(Rps::Scissors),
            _ => Err(Error::CharParse(
                "char not found for rock paper scissors".to_string(),
            )),
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Rps::Paper => match other {
                Rps::Paper => Some(Ordering::Equal),
                Rps::Rock => Some(Ordering::Greater),
                Rps::Scissors => Some(Ordering::Less),
            },
            Rps::Rock => match other {
                Rps::Paper => Some(Ordering::Less),
                Rps::Rock => Some(Ordering::Equal),
                Rps::Scissors => Some(Ordering::Greater),
            },

            Rps::Scissors => match other {
                Rps::Paper => Some(Ordering::Greater),
                Rps::Rock => Some(Ordering::Less),
                Rps::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

pub fn day_2() -> Result<DayCommand> {
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
        .name("day_2")
        .parts(&mut parts)
        .about("the solution to the day 2 advent of code problem")
        .build()
}

pub fn part_1(args: ArgMatches) -> Result<()> {
    let path: String = args.get_one::<String>("file").unwrap().to_owned();

    let path = Path::new(&path);

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut not_eof: bool = reader.read_line(&mut line)? != 0;

    let mut score = 0;

    while not_eof {
        let chars: Vec<_> = line.chars().filter(|x| x.is_alphabetic()).collect();
        let (elf_move, player_move) = (chars[0], chars[1]);

        let elf_move = Rps::try_from(elf_move)?;
        let player_move = Rps::try_from(player_move)?;

        match &player_move {
            Rps::Rock => score += 1,
            Rps::Paper => score += 2,
            Rps::Scissors => score += 3,
        }

        if player_move > elf_move {
            score += 6;
        } else if player_move == elf_move {
            score += 3;
        }

        line.clear();
        if reader.read_line(&mut line)? == 0 {
            not_eof = false;
        }
    }

    println!("The score for the player is {}", score);

    Ok(())
}

pub fn part_2(args: ArgMatches) -> Result<()> {
    let path: String = args.get_one::<String>("file").unwrap().to_owned();

    let path = Path::new(&path);

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut not_eof: bool = reader.read_line(&mut line)? != 0;

    let mut score = 0;

    while not_eof {
        let chars: Vec<_> = line.chars().filter(|x| x.is_alphabetic()).collect();
        let (elf_move, player_move) = (chars[0], chars[1]);

        let elf_move = Rps::try_from(elf_move)?;

        let player_move = Rps::corrected_read(player_move, &elf_move)?;

        match &player_move {
            Rps::Rock => score += 1,
            Rps::Paper => score += 2,
            Rps::Scissors => score += 3,
        }

        if player_move > elf_move {
            score += 6;
        } else if player_move == elf_move {
            score += 3;
        }

        line.clear();
        if reader.read_line(&mut line)? == 0 {
            not_eof = false;
        }
    }
    println!("The score for the player is {}", score);

    Ok(())
}
