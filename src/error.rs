use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub enum GError {
    CommError,
    IpcError,
    ModelUninit,
}

impl fmt::Display for GError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommError => write!(f, "Error in channel"),
            Self::IpcError => write!(f, "Erro while communicating with process"),
            Self::ModelUninit => write!(f, "Model used before initializing"),
        }
    }
}

impl Context for GError {}
