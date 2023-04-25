use crate::cfg::{get_cfg, OptEnvPath, OptPath, OptStr};
use getset::Getters;
use glossa::assets::OnceCell;
use serde::{Deserialize, Serialize};
pub(crate) mod dir;
mod sample;

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[serde(default)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct CoreCfg<'p> {
    cmt: OptStr,
    dir: CoreDir<'p>,
    logging: LogCfg,
}

#[derive(Deserialize, Serialize, Debug, Getters, Clone)]
#[serde(default)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct LogCfg {
    cmt: OptStr,
    #[serde(rename = "show-location")]
    show_location: bool,
    #[serde(rename = "stderr-level")]
    stderr_level: String,
    #[serde(rename = "file-level")]
    file_level: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[serde(default)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct CoreDir<'p> {
    cmt: OptStr,
    data: OptPath,
    #[serde(rename = "data-path")]
    data_path: OptEnvPath<'p>,
    cache: OptPath,
    #[serde(rename = "cache-path")]
    cache_path: OptEnvPath<'p>,
    #[serde(rename = "download")]
    dl: OptPath,
    #[serde(rename = "download-path")]
    dl_path: OptEnvPath<'p>,
}

pub(crate) fn get_static_core_cfg<'p>() -> &'static CoreCfg<'p> {
    static CFG: OnceCell<CoreCfg> = OnceCell::new();
    CFG.get_or_init(|| get_cfg("core.toml", None))
}

#[cfg(test)]
mod tests {
    use crate::cfg::core::CoreCfg;

    #[test]
    fn ser_core_cfg() -> anyhow::Result<()> {
        let a = CoreCfg::default();
        let s = toml::to_string_pretty(&a)?;
        println!("{s}");

        let cfg = toml::from_str::<CoreCfg>(&s)?;

        dbg!(cfg);
        Ok(())
    }

    #[test]
    fn ser_to_ron() -> anyhow::Result<()> {
        // let a = CoreCfg::default();
        // let s = ron::ser::to_string_pretty(&a, ron::ser::PrettyConfig::new())?;
        // println!("{s}");
        let b = ron::from_str::<CoreCfg>("()")?;
        dbg!(b);
        Ok(())
    }
}
