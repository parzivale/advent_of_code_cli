pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;
pub use crate::utils::{DayCommand, DayCommandBuilder, PartBuilder};

pub use std::fs::File;
pub use std::io::{BufRead, BufReader};
pub use std::path::Path;

pub use clap::ArgMatches;
