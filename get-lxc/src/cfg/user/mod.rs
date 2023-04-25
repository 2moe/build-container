use crate::cfg::{get_cfg, OptPath, OptStr, OptVecStr};
use get_lxc_core::cfg::{CodeKey, OsName};
use getset::Getters;
use glossa::assets::{HashMap, OnceCell};
use serde::{Deserialize, Serialize};

type AliasMap = HashMap<String, OsName>;
type LongAliasMap = HashMap<String, CodeKey>;

type ProfileMap = HashMap<String, ProfileAttr>;
mod sample;

pub(crate) fn get_static_user_cfg() -> &'static UserCfg {
    static CFG: OnceCell<UserCfg> = OnceCell::new();
    CFG.get_or_init(|| get_cfg("cfg.toml", None))
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[serde(default)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct UserCfg {
    #[serde(default = "sample::new_cmt")]
    cmt: String,
    os: OsMap,
    arch: ArchAttr,
    codename: CodeNameAttr,
    variant: VariantAttr,
    profile: ProfileMap,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct ProfileAttr {
    cmt: OptStr,
    inherits: OptStr,
    src: OptStr,
    mirror: OptStr,
    dir: ProfileDir,
    img: Option<ImgAttr>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct ImgAttr {
    cmt: OptStr,
    os: OptStr,
    codename: OptStr,
    arch: OptStr,
    variant: OptStr,
    tag: OptStr,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct ProfileDir {
    cmt: OptStr,
    #[serde(rename = "download")]
    dl: OptPath,
    #[serde(rename = "download-path")]
    dl_path: OptVecStr,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct OsMap {
    cmt: OptStr,
    alias: AliasMap,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct ArchAttr {
    cmt: OptStr,
    alias: AliasMap,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct CodeNameAttr {
    cmt: OptStr,
    alias: LongAliasMap,
}

#[derive(Deserialize, Serialize, Debug, Getters, Default, Clone)]
#[getset(get = "pub(crate) with_prefix")]
#[serde(default)]
pub(crate) struct VariantAttr {
    cmt: OptStr,
    alias: AliasMap,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_to_toml() -> anyhow::Result<()> {
        let cfg = toml::from_str::<UserCfg>("")?;
        let str = toml::ser::to_string_pretty(&cfg)?;

        println!("{str}");
        Ok(())
    }

    #[test]
    fn ser_to_ron() -> anyhow::Result<()> {
        let cfg = ron::from_str::<UserCfg>("()")?;
        let str = ron::ser::to_string_pretty(
            &cfg,
            ron::ser::PrettyConfig::new()
                .extensions(ron::extensions::Extensions::IMPLICIT_SOME),
        )
        .unwrap();

        println!("{str}");
        Ok(())
    }
}
