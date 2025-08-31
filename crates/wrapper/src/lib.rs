use std::error::Error;
use std::fmt;

mod library;
mod builder;

pub use library::Library;
pub use builder::LibraryBuilder;

/// Error type for wrapper operations
#[derive(Debug)]
pub enum WrapperError {
    LibraryLoad(String),
    FunctionCall(String),
    ValueAccess(String),
}

impl fmt::Display for WrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WrapperError::LibraryLoad(msg) => write!(f, "Library load error: {}", msg),
            WrapperError::FunctionCall(msg) => write!(f, "Function call error: {}", msg),
            WrapperError::ValueAccess(msg) => write!(f, "Value access error: {}", msg),
        }
    }
}

impl Error for WrapperError {}

pub type Result<T> = std::result::Result<T, WrapperError>;

/// Represents a loaded Koto library with a Rust-friendly interface
pub struct LibraryLoader {
    _private: (),
}

impl LibraryLoader {
    /// Load a dynamic library from path, returning a Library instance
    pub fn load<P: AsRef<str>>(path: P) -> Result<Library> {
        Library::load(path)
    }
}