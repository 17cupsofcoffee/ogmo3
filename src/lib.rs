//! `ogmo3` is a Rust crate for parsing projects and levels created with [Ogmo Editor 3](https://ogmo-editor-3.github.io/).

#![warn(missing_docs)]

pub mod level;
pub mod project;

use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::io;

use serde::{Deserialize, Serialize};

pub use level::{Layer, Level, Value};
pub use project::Project;

/// The various kinds of errors that can occur while parsing Ogmo data.
#[derive(Debug)]
pub enum Error {
    /// An IO error was encountered.
    Io(io::Error),

    /// An error was encountered while deserializing JSON.
    Json(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(_) => write!(f, "IO error"),
            Error::Json(_) => write!(f, "JSON error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(cause) => Some(cause),
            Error::Json(cause) => Some(cause),
        }
    }
}

/// An X and Y value.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Vec2<T> {
    /// The X component.
    pub x: T,

    /// The Y component.
    pub y: T,
}
