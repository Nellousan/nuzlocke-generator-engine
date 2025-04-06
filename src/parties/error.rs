use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PartyError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
}
