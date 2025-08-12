// src/error.rs
use std::error::Error;
use std::fmt::{self, Display, Formatter};

use clap::builder::Str;

#[derive(Debug)]
pub enum FlowError {
    Message(String),
    Network(String),
    Ssid(String),
    Io(std::io::Error),
    EncryptionError(String),
    FileError(String),
    PermissionError(String),
    ParsingError(String),
}

impl Display for FlowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FlowError::Message(msg) => write!(f, "{}", msg),
            FlowError::Network(msg) => write!(f, "Network error: {}", msg),
            FlowError::Ssid(msg) => write!(f, "Ssid error: {}", msg),
            FlowError::Io(err) => write!(f, "I/O error: {}", err),
            FlowError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            FlowError::FileError(msg) => write!(f, "File related error: {}", msg),
            FlowError::PermissionError(msg) => write!(f, "Permission error: {}", msg),
            FlowError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

impl Error for FlowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FlowError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FlowError {
    fn from(err: std::io::Error) -> Self {
        FlowError::Io(err)
    }
}

impl From<&str> for FlowError {
    fn from(s: &str) -> Self {
        FlowError::Message(s.to_string())
    }
}

impl From<String> for FlowError {
    fn from(s: String) -> Self {
        FlowError::Message(s)
    }
}
