use crate::prelude::*;

#[derive(Clone)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

pub enum State {
    Won,
    Lost,
    Draw,
}

impl Rps {
    pub fn corrected_read(c: char, elf_move: &Rps) -> Result<Self, &'static str> {
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
                return Err(
                    "char not found for rock paper scissors"
                )
            }
        };
        Ok(t)
    }
}

impl TryFrom<char> for Rps {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, &'static str> {
        match c {
            'A' => Ok(Rps::Rock),
            'B' => Ok(Rps::Paper),
            'C' => Ok(Rps::Scissors),
            'X' => Ok(Rps::Rock),
            'Y' => Ok(Rps::Paper),
            'Z' => Ok(Rps::Scissors),
            _ => Err("char not found for rock paper scissors"),
        }
    }
}

impl Rps {
    fn has_won(&self, other: &Self) -> State {
        match self {
            Rps::Paper => match other {
                Rps::Paper => State::Draw,
                Rps::Rock => State::Won,
                Rps::Scissors => State::Lost,
            },
            Rps::Rock => match other {
                Rps::Paper => State::Lost,
                Rps::Rock => State::Draw,
                Rps::Scissors => State::Won,
            },

            Rps::Scissors => match other {
                Rps::Paper => State::Won,
                Rps::Rock => State::Lost,
                Rps::Scissors => State::Draw,
            },
        }
    }
}

pub fn day_2() -> BoxResult<DayCommand> {
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

pub fn part_1(args: ArgMatches) -> BoxResult<()> {
    let f = FileReader::try_from(args)?;

    let mut score = 0;

    for i in f {
        let chars: Vec<_> = i.chars().filter(|x| x.is_alphabetic()).collect();
        let (elf_move, player_move) = (chars[0], chars[1]);

        let elf_move = Rps::try_from(elf_move)?;
        let player_move = Rps::try_from(player_move)?;

            match &player_move {
                Rps::Rock => score += 1,
                Rps::Paper => score += 2,
                Rps::Scissors => score += 3,
            }

            match player_move.has_won(&elf_move){
                State::Won => score += 6,
                State::Draw => score += 3,
                _ => {}
            }
    }

    println!("The score for the player is {}", score);

    Ok(())
}

pub fn part_2(args: ArgMatches) -> BoxResult<()> {
    let f = FileReader::try_from(args)?;

    let mut score = 0;

    for i in f {
        let chars: Vec<_> = i.chars().filter(|x| x.is_alphabetic()).collect();
        let (elf_move, player_move) = (chars[0], chars[1]);

        let elf_move = Rps::try_from(elf_move)?;

        let player_move = Rps::corrected_read(player_move, &elf_move)?;

        match &player_move {
            Rps::Rock => score += 1,
            Rps::Paper => score += 2,
            Rps::Scissors => score += 3,
        }

        match player_move.has_won(&elf_move){
            State::Won => score += 6,
            State::Draw => score += 3,
            _ => {}
        }
    }
    println!("The score for the player is {}", score);

    Ok(())
}
