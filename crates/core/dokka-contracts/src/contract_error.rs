use std::error::Error;
use std::fmt;

/// Fallo al cumplir un contrato.
#[derive(Debug)]
pub enum ContractError {
    NotFound,
    Failed(String),
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => f.write_str("elemento no encontrado"),
            Self::Failed(reason) => write!(f, "la operación falló: {reason}"),
        }
    }
}

impl Error for ContractError {}
