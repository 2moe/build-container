use std::{
    ffi::OsStr,
    fs::{self, File},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

pub(crate) fn extract_specific_level_tgz<P: AsRef<Path>>(
    src_file: P,
    dst: Option<PathBuf>,
    dst_dir_name: Option<&str>,
    min: usize,
    max: Option<usize>,
) -> std::io::Result<()> {
    let file = File::open(&src_file)?;
    let gz_decoder = GzDecoder::new(file);
    let mut tar = Archive::new(gz_decoder);

    let dst_dir = dst.unwrap_or_else(|| {
        src_file
            .as_ref()
            .parent()
            .unwrap_or(Path::new("."))
            .join(dst_dir_name.unwrap_or(""))
    });

    fs::create_dir_all(&dst_dir)?;

    #[allow(unused_assignments)]
    let mut epath = PathBuf::with_capacity(dst_dir.capacity() + max.unwrap_or(10));

    println!("dir: {}\nfile(s):", dst_dir.display());

    for mut entry in tar
        .entries()?
        .filter_map(Result::ok)
        .filter(|e| {
            // matches!(e.path(), Ok(p) if p.components().count() >= min )
            e.path().map_or(false, |p| {
                let lv = p.components().count();
                (min..=max.unwrap_or(usize::MAX - 2)).contains(&lv)
            })
        })
    {
        epath = entry
            .path()?
            .iter()
            .enumerate()
            .map(|(i, p)| if i == 0 { dst_dir.as_os_str() } else { p })
            .collect::<PathBuf>();

        println!(
            "{:?}",
            epath
                .file_name()
                .unwrap_or_else(|| OsStr::new(""))
        );

        match entry.header().entry_type() {
            t if t.is_dir() => fs::create_dir_all(&epath)?,
            _ => {
                entry.unpack(epath)?;
            }
        }
    }
    Ok(())
}
