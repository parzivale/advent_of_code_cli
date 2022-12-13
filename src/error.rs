use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Clap(#[from] clap::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Fail to create day. Cause: {0}")]
    CommandBuilder(String),

    #[error("Failed to run day. Cause: {0}")]
    CommandRunner(String),

    #[error(transparent)]
    NumParse(#[from] std::num::ParseIntError),

    #[error("Failed to convert char to type: {0}")]
    CharParse(String),

    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),
}
