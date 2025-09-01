use core::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type Result<T> = core::result::Result<T, TranslationError>;

/// Error type for translation operations.
///
/// This errpr is used by translation operations using
/// `Transmute` and `FromBytes` traits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationError {
    /// Invalid length for translation.
    InvalidLength,

    /// Invalid value found for type.
    InvalidValue,

    /// Misaligned memory for translation.
    Misaligned,
}

impl Error for TranslationError {}

impl Display for TranslationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TranslationError::InvalidLength => write!(f, "invalid length for translation"),
            TranslationError::InvalidValue => write!(f, "invalid value found for type"),
            TranslationError::Misaligned => write!(f, "misaligned memory for translation"),
        }
    }
}
