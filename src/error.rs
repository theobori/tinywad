use thiserror::Error;

#[derive(Error, Debug)]
pub enum WadError {
    #[error("Unable to read this file: {0}")]
    Read(String),
    #[error("Wrong WAD file type: {0}")]
    Type(&'static str),
    #[error("Unable to load the program: {0}")]
    Load(&'static str),
    #[error("Invalid lump name")]
    InvalidLumpName,
    #[error("Invalid Regex")]
    InvalidRegex,
    #[error("Invalid operation")]
    InvalidOperation,
    #[error("Unable to use this API")]
    Unknown
}
