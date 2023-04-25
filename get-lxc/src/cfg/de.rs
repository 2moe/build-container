use crate::error::Error;
use get_lxc_core::{cfg::Root, error::FileError};
use std::{fs, path::Path};

type RonResult<T, E = Error> = Result<T, E>; // The default error is RonError.

/// Deserialize an ron configuration file into a Root struct
///
/// # Examples
///
/// ```no_run
/// use get_lxc_core::cfg::Root;
/// use std::path::Path;
///
/// let path = Path::new("config.ron");
/// let cfg = de_ron::<Root>(path)?;
///
/// dbg!(&cfg);
/// ```
pub(crate) fn get_lxc_root_cfg<P>(path: P) -> RonResult<Root>
where
    P: AsRef<Path>,
{
    // Convert the input path to a PathBuf
    let path = path.as_ref();
    // Read the contents of the file at the specified path into a string
    let cfg_str = fs::read_to_string(path)
        // If there was an error reading the file, convert it to a RonError and return it
        .map_err(|err| Error::Read(FileError::new(err, path)))?;

    // Attempt to deserialize the ron cfg from the &str
    Ok(ron::from_str(&cfg_str)?)
}
