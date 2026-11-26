use std::error::Error;
use std::fmt;

/// Fallo del compositor de Windows.
#[derive(Debug)]
pub struct CompositionError(windows::core::Error);

impl From<windows::core::Error> for CompositionError {
    fn from(error: windows::core::Error) -> Self {
        Self(error)
    }
}

impl fmt::Display for CompositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error de composición: {}", self.0)
    }
}

impl Error for CompositionError {}
