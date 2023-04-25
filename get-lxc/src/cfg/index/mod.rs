use std::collections::HashSet;

use crate::cfg::Sample;
use getset::Getters;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Getters, Default, Clone)]
#[serde(default)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct LxcIndex {
    list: HashSet<String>,
    time: LxcIndexTime,
}

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[serde(default)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct LxcIndexTime {
    #[serde(rename = "last-update")]
    #[serde(with = "time::serde::rfc3339")]
    last_update: OffsetDateTime,
}

impl Default for LxcIndexTime {
    fn default() -> Self {
        Self {
            last_update: OffsetDateTime::now_utc() - time::Duration::days(366),
        }
    }
}

impl Sample for LxcIndex {}

// pub(crate) fn get_lxc_index_cfg<'p>() {
//     static CFG: OnceCell<CoreCfg> = OnceCell::new();
//     get_cfg("core.toml", None)
// }
