use std::{io, path::PathBuf};
use thiserror::Error;

/// Re-exports the `tinystr::TinyStrError` error type as a public module with an alias of `StrError`.
/// This allows users of this crate to access the `TinyStrError` type under its alias as a module under the parent module.
pub use tinystr::TinyStrError as StrErr;

/// The `FileError` type represents an error that occurs during file operations.
#[derive(Error, Debug)]
#[error("File: {}", path.display())]
pub struct FileError {
    /// The source of the error.
    #[source]
    src: io::Error,
    /// The path of the file that the operation was performed on.
    path: PathBuf,
}

impl FileError {
    /// Create a new `FileError` instance with the given `std::io::Error` source and `PathBuf` path.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::{
    ///     fs::File,
    ///     io::Error,
    ///     path::Path,
    /// };
    /// use get_lxc_core::error::FileError;
    ///
    /// let path = Path::new("Non-existent-document.txt");
    /// let _file = File::open(path).map_err(|err| FileError::new(err, path)).expect("Failed to open the file");
    /// ```
    ///
    /// The above code will produce the following output in case an error occurs:
    ///
    /// ```text
    /// stderr:
    /// Failed to open the file: FileError { src: Os { code: 2, kind: NotFound, message: "No such file or directory" }, path: "Non-existent-document.txt" }
    /// ```
    pub fn new<P: Into<PathBuf>>(src: io::Error, path: P) -> Self {
        Self {
            src,
            path: path.into(),
        }
    }
}
