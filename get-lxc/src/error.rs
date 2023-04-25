use get_lxc_core::error::{FileError, StrErr};
use std::io;
use thiserror::Error;

/// This enum represents errors that may occur during Ron parsing operations
#[derive(Debug, Error)]
pub(crate) enum Error {
    /// Represents the error that occurs while reading the file
    #[error("Failed to read file.")]
    Read(#[from] FileError),

    /// Represents any Generic IO Errors
    #[error("I/O Error.")]
    IO(
        #[from]
        #[source]
        io::Error,
    ),

    /// Represents the error that occurs while deserializing Ron cfg
    #[error("Failed to deserialize ron cfg.")]
    RonSpanned(
        #[from]
        #[source]
        ron::de::SpannedError,
    ),

    /// Represents the error that occurs while parsing TinyStr
    #[error("Failed to parse tinystr.")]
    TinyStrParse(
        #[from]
        #[source]
        StrErr,
    ),

    /// Represents any unhandled/unknown errors
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
