use std::error::*;
pub type BoxResult<T> = std::result::Result<T, Box<dyn Error>>;
pub use crate::utils::{DayCommand, DayCommandBuilder, FileReader, PartBuilder};

pub use std::fs::File;
pub use std::io::{BufRead, BufReader};
pub use std::path::Path;

pub use clap::ArgMatches;
