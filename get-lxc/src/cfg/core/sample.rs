use crate::cfg::{
    core::{CoreCfg, CoreDir, LogCfg},
    new_opt_str, Sample,
};
use envpath::{get_pkg_name, EnvPath};
use std::path::PathBuf;

impl<'p> Sample for CoreCfg<'p> {
    fn sample() -> Self {
        let name = get_pkg_name!();
        let cmt = Some("This is the core configuration.".to_owned());

        let dir = CoreDir {
            data: Some(PathBuf::new()),
            data_path: Some(EnvPath::from_iter([format!(
                "$proj  (me. tmoe. {name}):  cli-data?  local-data",
            )])),
            cache: Some(PathBuf::new()),
            cache_path: Some(EnvPath::from_iter([format!(
                "$proj  (me. tmoe. {name}):  cli-cache ?  cache",
            )])),
            dl: Some(PathBuf::new()),
            dl_path: Some(EnvPath::from_iter([
                "$dir: dl ?     env * XDG_DOWNLOAD_DIR",
                name,
            ])),
            ..Default::default()
        };

        Self {
            cmt,
            dir,
            ..Default::default()
        }
    }
}

impl Default for LogCfg {
    fn default() -> Self {
        Self {
            cmt: new_opt_str("This is where the logs are recorded"),
            show_location: true,
            stderr_level: "info".to_owned(),
            file_level: "warn".to_owned(),
        }
    }
}
