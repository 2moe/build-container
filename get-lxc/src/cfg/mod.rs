use crate::cfg::core::dir::get_static_cfg_dir;
use envpath::EnvPath;
use glossa::log::{err, warning};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

pub(crate) mod core;
pub(crate) mod de;
pub(crate) mod index;
pub(crate) mod user;

type OptStr = Option<String>;
// type VecStr = Vec<String>;
type OptVecStr = Option<Vec<String>>;
type OptPath = Option<PathBuf>;
type OptEnvPath<'p> = Option<EnvPath<'p>>;

pub(crate) fn add_ext_if_not_exists(p: &Path) -> Cow<Path> {
    match p.exists() {
        true => Cow::from(p),
        _ => {
            match p
                .extension()
                .and_then(|s| s.to_str())
            {
                Some(_) => Cow::from(p),
                _ => Cow::from(p.with_extension("toml")),
            }
        }
    }
}

// get-lxc c go.www
pub(crate) fn get_cfg<T: Sample + Serialize + DeserializeOwned>(
    fname: &str,
    custom: Option<&Path>,
) -> T {
    let join_file = || Cow::from(get_static_cfg_dir().join(fname));

    let file = custom.map_or_else(join_file, add_ext_if_not_exists);

    let sample = || create_cfg_sample(&file);

    if !file.exists() {
        return sample();
    }

    match fs::read_to_string(&file) {
        Ok(s) if s.trim().is_empty() => sample(),
        Ok(ref s) => toml::from_str(s).unwrap_or_else(|e| {
            err!("{e}");
            sample()
        }),
        Err(e) => {
            warning!("{e}");
            sample()
        }
    }
}
pub(crate) trait Sample: Default {
    fn sample() -> Self {
        Default::default()
    }
}

fn create_cfg_sample<T: Serialize + Sample>(file: &Path) -> T {
    let cfg = T::sample();

    if file.is_dir() {
        return cfg;
    }

    if let Some(p) = file.parent() {
        fs::create_dir_all(p).expect("Unable to create a folder!")
    }

    toml::to_string_pretty(&cfg)
        .map(|s| {
            fs::write(file, s).unwrap_or_else(|e| {
                eprintln!("Failed to write to core.toml, err: {e}")
            })
        })
        .unwrap_or_else(|e| {
            warning!("Unable to serialize data structures to text strings in toml format!{e}")
        });

    cfg
}

fn new_opt_str(s: &str) -> OptStr {
    Some(s.to_owned())
}

fn some_path_buf() -> OptPath {
    Some(PathBuf::new())
}

fn map_to_opt_vec(arr: &[&str]) -> OptVecStr {
    Some(
        arr.iter()
            .map(|x| x.to_string())
            .collect(),
    )
}

#[test]
fn test_none() {
    let a: OptStr = Default::default();
    // let b = opt_none();
    dbg!(a);
}
