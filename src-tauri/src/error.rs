use std::fmt;

/// Errors surfaced across the Tauri boundary.
///
/// The release profile sets `panic = "abort"`, so unwrapping on user input
/// would kill the app with no message.
#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum Error {
    Read(String),
    Write(String),
    UnknownSheet(String),
    MalformedOp(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(message) => write!(f, "could not read workbook: {message}"),
            Self::Write(message) => write!(f, "could not write workbook: {message}"),
            Self::UnknownSheet(name) => write!(f, "unknown sheet: {name}"),
            Self::MalformedOp(message) => write!(f, "malformed operation: {message}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
