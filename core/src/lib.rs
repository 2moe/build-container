pub mod cfg;
pub mod error;

#[cfg(feature = "request")]
pub mod fetch;

#[cfg(feature = "request")]
pub use reqwest;
use tinystr::TinyAsciiStr;

pub mod table;

pub const fn as_tiny_str<const N: usize>(s: &str) -> TinyAsciiStr<N> {
    match TinyAsciiStr::from_str(s) {
        Ok(x) => x,
        _ => panic!("Failed to convert as tinystr"),
    }
}
