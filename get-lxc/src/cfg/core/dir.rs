use crate::{
    assets::locales,
    cfg::{core::get_static_core_cfg, OptEnvPath, OptPath},
};
use envpath::{get_pkg_name, EnvPath, ProjectDirs};
use getset::Getters;
use glossa::{
    assets::{
        log::{info, warn},
        OnceCell,
    },
    GetText,
};
use std::{
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};
type Proj = ProjectDirs;

fn get_proj_dir() -> io::Result<&'static Proj> {
    static PROJ: OnceCell<Proj> = OnceCell::new();
    PROJ.get_or_try_init(|| {
        info!("{}", locales().get_cow("dir", "get_proj_dir")?);
        EnvPath::new_project("me", "tmoe", get_pkg_name!())
    })
}

pub(crate) fn get_static_cfg_dir() -> &'static PathBuf {
    static CFG_DIR: OnceCell<PathBuf> = OnceCell::new();
    CFG_DIR.get_or_init(|| {
        env::var_os("TMM_CFG_HOME")
            .map(|o| {
                PathBuf::from_iter([o.as_os_str(), OsStr::new(get_pkg_name!())])
            })
            .unwrap_or_else(|| match get_proj_dir() {
                Ok(x) => x
                    .config_local_dir()
                    .to_path_buf(),
                Err(e) => {
                    warn!("{e}");
                    let mut cur = env::current_dir().unwrap_or_default();
                    cur.extend([".local", "share", get_pkg_name!()]);
                    cur
                }
            })
    })
}

fn is_valid_dir(p: &Path) -> bool {
    if p.as_os_str().is_empty() {
        return false;
    }
    matches!(fs::create_dir_all(p), Ok(_) if p.is_dir())
}

#[derive(Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct DataDir<'a> {
    data: &'a Path,
    cache: &'a Path,
    dl: &'a Path,
}

pub(crate) fn get_static_data_dir<'a>() -> &'static DataDir<'a> {
    static DIR: OnceCell<DataDir> = OnceCell::new();
    DIR.get_or_init(|| {
        let cfg = get_static_core_cfg();

        let data = get_data_cfg(
            cfg.dir.get_data(),
            cfg.dir.get_data_path(),
            Proj::data_local_dir,
        );

        let cache = get_data_cfg(
            cfg.dir.get_cache(),
            cfg.dir.get_cache_path(),
            Proj::cache_dir,
        );

        let dl =
            get_data_cfg(cfg.dir.get_dl(), cfg.dir.get_dl_path(), Proj::cache_dir);

        DataDir { data, cache, dl }
    })
}

fn get_data_cfg<'a, F>(opt: &'a OptPath, env_path: &'a OptEnvPath, f: F) -> &'a Path
where
    F: Fn(&Proj) -> &Path,
{
    let proj = || get_proj_dir().expect("Failed to get project dir");

    match opt.as_deref() {
        Some(p) if is_valid_dir(p) => p,
        _ => match env_path.as_deref() {
            Some(p) if is_valid_dir(p) => p,
            _ => f(proj()),
        },
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use envpath::EnvPath;
    #[test]
    fn my_pkg_name() {
        let p = EnvPath::new(["$dir: tmp-rand"]);
        dbg!(p);
    }
}
