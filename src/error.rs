use thiserror::Error;

#[derive(Error, Debug)]
pub enum PingyinError {
    #[error("parse {0} error occurred")]
    ParseStrError(String),
}
