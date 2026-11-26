use std::error::Error;
use std::fmt;

/// Fallo de Windows al preparar o atender una ventana.
#[derive(Debug)]
pub struct SurfaceError(windows::core::Error);

impl From<windows::core::Error> for SurfaceError {
    fn from(error: windows::core::Error) -> Self {
        Self(error)
    }
}

impl fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error de ventana: {}", self.0)
    }
}

impl Error for SurfaceError {}
