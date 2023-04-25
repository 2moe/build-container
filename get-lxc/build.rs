use glossa_codegen::{consts::*, prelude::*};
use std::{
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    let ver = get_pkg_version!();

    // Create a new `PathBuf` from the result of calling `get_l10n_rs_file_arr()`
    let mut path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&path, Some(ver))? {
        return Ok(());
    }

    append_to_l10n_mod(&path)?;

    let file = BufWriter::new(File::create(&path)?);
    let mut writer = MapWriter::new(file);

    *writer.get_visibility_mut() = "pub(super)";
    // Update the `PathBuf` to point to the directory containing the localisation data
    path = PathBuf::from_iter(
        [".."]
            .into_iter()
            .chain(default_l10n_dir_arr()),
    );
    let generator = Generator::new(path).with_version(ver);

    generator.run(writer)
}
