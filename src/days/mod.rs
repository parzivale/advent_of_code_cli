use crate::{prelude::*, utils::DayCommand};

mod day_4;
use day_4::*;

pub fn generate_days<'a>() -> Result<Vec<DayCommand<'a>>> {
    let days = vec![day_4()?];
    Ok(days)
}
