use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DayCommandError {
    #[error("command part {0} not found")]
    CommandPartNotFound(String),
}

#[derive(Error, Debug)]
pub enum DayCommandBuilderError {
    #[error("name was not specified")]
    NameNotFound,

    #[error("command parts were not specified")]
    PartsNotFound,
}
