use reqwest::Url;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncWriteExt, BufWriter},
};

const MIB: usize = 1024 * 1024;
const MIB_8: u64 = 8 * (MIB as u64);
// const MIB_16: usize = 16 * MIB;

// Define a struct for asynchronous file handling
pub(crate) enum AsyncFile {
    File(File),
    Writer(BufWriter<File>),
}

impl AsyncFile {
    // Write a chunk of bytes to the asynchronous file.
    pub(crate) async fn write_all(&mut self, chunk: &[u8]) -> io::Result<()> {
        match self {
            AsyncFile::Writer(w) => w.write_all(chunk).await,
            AsyncFile::File(f) => f.write_all(chunk).await,
        }
    }
    // Flush the asynchronous file.
    pub(crate) async fn flush(&mut self) -> io::Result<()> {
        match self {
            AsyncFile::Writer(w) => w.flush().await,
            AsyncFile::File(f) => f.flush().await,
        }
    }
    pub(crate) fn new(size: u64, file: File) -> Self {
        match size {
            0..=MIB_8 => AsyncFile::File(file),
            _ => AsyncFile::Writer(BufWriter::with_capacity(8 * MIB, file)),
        }
    }
}

/// Create or open a file in append mode
pub(crate) async fn create_or_open_file<P: AsRef<Path>>(
    fname: P,
    local_size: u64,
    full_size: u64,
) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(local_size == 0)
        .append(local_size <= full_size && local_size > 0 && full_size != 0)
        .open(fname)
        .await
}

/// Get the size of a local file, if it exists
pub(crate) fn get_local_file_size<P: AsRef<Path>>(
    file_name: P,
) -> std::io::Result<u64> {
    let p = file_name.as_ref();
    Ok(match p {
        p if p.exists() => p.metadata()?.len(),
        _ => 0,
    })
}

/// Get the filename from a URL
pub(crate) fn get_filename(url: &Url) -> Cow<Path> {
    url.path_segments()
        .and_then(|sgmts| sgmts.last())
        .filter(|x| !x.is_empty())
        .map(|x| match x {
            x if x.contains('%') => Cow::from(PathBuf::from(
                form_urlencoded::parse(x.as_bytes())
                    .map(|(k, v)| [k, v].concat())
                    .collect::<String>(),
            )),
            _ => Cow::Borrowed(Path::new(x)),
        })
        .unwrap_or(Cow::Borrowed(Path::new("index.html")))
}
