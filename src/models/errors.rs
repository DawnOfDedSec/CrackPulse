use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid command line arguments: {0}")]
    CantReadUserInput(io::Error),
    #[error("User gave Invalid Input: {0}")]
    InvalidUserInput(String),
}
