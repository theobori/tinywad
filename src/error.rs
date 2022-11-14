use thiserror::Error;

#[derive(Error, Debug)]
pub enum WadError {
    #[error("Unable to read this file: {0}")]
    Read(String),
    #[error("Unable to load the program")]
    Load,
    #[error("Unable to use this API")]
    Unknown
}
