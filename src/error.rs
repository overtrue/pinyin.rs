use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PinyinError {
    #[error("input is too long: {actual} bytes, max {max} bytes")]
    InputTooLong { actual: usize, max: usize },

    #[error("invalid max input length: {0}")]
    InvalidMaxInputLength(usize),

    #[error("invalid permalink delimiter: {0:?}")]
    InvalidDelimiter(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, PinyinError>;

impl PinyinError {
    pub(crate) fn invalid_delimiter(delimiter: &str) -> Self {
        Self::InvalidDelimiter(delimiter.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_errors() {
        assert_eq!(
            PinyinError::InputTooLong { actual: 10, max: 3 }.to_string(),
            "input is too long: 10 bytes, max 3 bytes"
        );
        assert_eq!(
            PinyinError::invalid_delimiter("=").to_string(),
            "invalid permalink delimiter: \"=\""
        );
    }
}
