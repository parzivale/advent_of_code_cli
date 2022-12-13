use crate::{prelude::*, utils::DayCommand};

mod day_1;
mod day_2;
mod day_4;
use day_1::*;
use day_2::*;
use day_4::*;

pub fn generate_days() -> Result<Vec<DayCommand>> {
    let days = vec![day_4()?, day_1()?, day_2()?];
    Ok(days)
}
