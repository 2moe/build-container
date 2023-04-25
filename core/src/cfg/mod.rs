pub mod arch;

/// The ahash crate provides a high-performance hash map implementation that uses the AHash algorithm. It is an alternative to Rust's standard library HashMap. The AHashMap is a data structure that stores values as key-value pairs based on the hash value of the key. It allows fast lookups and insertions and is optimized for speed.
use ahash::AHashMap;

/// The getset crate provides macros to create getters and setters for Rust structs. The Getters macro generates immutable getters for struct fields, while CopyGetters generates copyable getters for fields that implement Copy. These macros make it easier to create an interface for accessing struct fields and improve code readability and maintainability.
use getset::{CopyGetters, Getters};

/// The serde crate provides a framework for serializing and deserializing Rust data structures. The Deserialize trait is used to deserialize data from a serialized format such as RON or TOML into a usable Rust data structure. It is commonly used when working with data coming from external systems, such as APIs or databases.
use serde::Deserialize;

/// The std::collections module contains various useful data structures provided by Rust's standard library. The BTreeMap is a data structure that stores values as key-value pairs in a sorted tree-like structure that allows for efficient lookups and insertions. It is particularly useful when it is necessary to iterate the keys in order or to retrieve a range of keys.
use std::collections::BTreeMap;

/// The tinystr crate provides a small, fixed-size string type optimized for ASCII character sets. It is useful when there is a need for a small string that can fit on the stack, and avoiding heap allocations can improve performance.
use tinystr::TinyAsciiStr;

/// Define a type alias for the code key.
pub type CodeKey = TinyAsciiStr<32>;

pub type OsName = TinyAsciiStr<24>;

/// Define a deserializable type called `Root` that represents the root configuration.
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Root {
    os: OsName, // Contains a TinyAsciiStr representing the operating system. (e.g. archlinux, ubuntu, springdalelinux)
    codename: AHashMap<CodeKey, Variant>, // Contains a hash map of variants, each identified by a TinyAsciiStr code key.
}

/// Define a type alias for the variant key.
pub type VariantKey = TinyAsciiStr<16>;

pub const fn set_variant_key(key: &str) -> VariantKey {
    match VariantKey::from_str(key) {
        Ok(x) => x,
        _ => panic!("Const panic! Failed to parse &str to VariantKey."),
    }
}

pub const fn variant_key_default() -> (&'static str, VariantKey) {
    let s = "default";
    (s, set_variant_key("default"))
}

pub const fn variant_key_cloud() -> (&'static str, VariantKey) {
    let s = "cloud";
    (s, set_variant_key(s))
}

/// Define a deserializable type called `Variant` that represents the variant configuration.
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Variant {
    variant: AHashMap<VariantKey, Architecture>, // Contains a hash map of architectures, each identified by a TinyAsciiStr variant key.
}

/// Define a type alias for the architecture key.
pub type ArchKey = TinyAsciiStr<16>;
/// Define a deserializable type called `Architecture` that represents the architecture configuration.
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Architecture {
    arch: AHashMap<ArchKey, Versions>, // Contains a hash map of versions for a given architecture, each identified by a TinyAsciiStr architecture key.
}

/// Define a type alias for the tag key.
pub type TagKey = TinyAsciiStr<16>;

/// Define a deserializable type called `Versions` that represents the version configuration.
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Versions {
    tags: BTreeMap<TagKey, RootFS>, // For sorting purposes, a BTreeMap is used instead of a HashMap.
                                    // A BTreeMap of root file systems (RootFS) for a given version, each identified by a TinyAsciiStr tag key.
}

/// Define a deserializable type called `RootFS` that represents a root file system configuration.
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct RootFS {
    #[serde(rename = "type")]
    // We add a serde tag to rename the `type` field to `ftype`. Note: `type` is a keyword in rust
    ftype: TinyAsciiStr<16>, // The file type, represented by a TinyAsciiStr.
    digest: Digest, // The hash digest of the file.
    size: Size,     // The size of the file.
    path: String,   // In fact, this is not a real file path, but a part of the uri.
}

/// Define a deserializable type called `Digest` that represents a hash digest for a file.
#[derive(Deserialize, Debug, CopyGetters, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Digest {
    algorithm: TinyAsciiStr<16>, // The algorithm used for the hash digest. (e.g. sha256, sha3-256, blake3)
    hex: TinyAsciiStr<64>, // The value of the hash digest, represented as a TinyAsciiStr.
}

/// Define a deserializable type called `Size` that represents the size of a file.
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Size {
    bytes: u64, // The size of the file in bytes.
    str: TinyAsciiStr<16>, // The size of the file as a TinyAsciiStr string.
                // The content inside is "humanly readable".(e.g. 114.51 MiB)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, fs,
        path::{Path, PathBuf},
    };

    #[test]
    fn get_os_name_len() {
        let a = std::mem::size_of::<OsName>();
        dbg!(a);
    }

    #[test]
    fn deserialize_cfg() -> anyhow::Result<()> {
        // Example configuration written in RON format.
        let ron_cfg = r#"(
            os: "Test",
            codename: {
                "current": (
                    variant: {
                        "default": (
                            arch: {
                                "amd64": (
                                    tags: {"2023-02-26 20:46": (type: "tar.zst", digest: (algorithm: "blake3", hex: "3c349579235b8bfb12912f5d224b6f644b46fcb29d5e00b602df25fd2cf5305c"), size: (bytes: 76656836, str: "73.11 MiB"), path: "images/pld/current/amd64/default/20230226_20:46/rootfs.tar.zst"), "2023-02-27 20:46": (type: "tar.xz", digest: (algorithm: "sha3-256", hex: "4c973730132408b0b5b2435ef64c89f511d16d37a3e912fe1628d833cc99437b"), size: (bytes: 76644596, str: "73.09 MiB"), path: "images/pld/current/amd64/default/20230227_20:46/rootfs.tar.xz"), "2023-02-28 20:46": (type: "tar.xz", digest: (algorithm: "sha256", hex: "6fe51da68c7fc3413e778686c8566ad4481a51d3ec09c733ac845ae30ec64815"), size: (bytes: 76657280, str: "73.11 MiB"), path: "images/pld/current/amd64/default/20230228_20:46/rootfs.tar.xz")},
                                ),
                            },
                        ),
                    },
                ),
            },
        )"#;

        // Print the current cargo manifest directory.
        dbg!(std::env::var("CARGO_MANIFEST_DIR")?);

        // Get the temporary directory and write the configuration to a file in that directory.
        let dir = env::temp_dir();
        let path = PathBuf::from_iter([&dir, Path::new("test.ron")]);
        fs::write(&path, ron_cfg)?;

        // Read the configuration from the file.
        let cfg_str = fs::read_to_string(&path)?;

        // Deserialize the RON configuration into a Root struct.
        let cfg = ron::from_str::<Root>(&cfg_str)?;

        // Extract the OS and codename(Map) from the configuration.
        let os = cfg.get_os();
        let codename = &cfg.codename;

        // Print the extracted values and return OK.
        dbg!(os, codename);
        Ok(())
    }
}
