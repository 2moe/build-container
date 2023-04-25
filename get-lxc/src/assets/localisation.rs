// Version: 0.0.0-alpha.2
#![allow(dead_code)]

use super::{lang_id_consts, HashMap, LangID};

pub(crate) type L10nMap = ::phf::Map<&'static str, &'static str>;
pub(crate) type SubLocaleMap = ::phf::Map<&'static str, fn() -> L10nMap>;
pub(crate) type LocaleMap = ::phf::Map<&'static str, fn() -> SubLocaleMap>;
pub(crate) type LocaleHashMap = HashMap<LangID, SubLocaleMap>;

/// Language ID: en;
/// Map name: "dir";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("dir", "get-proj-dir");
///
/// assert_eq!(msg, "Getting the project dir...");
/// ```
pub(super) const fn get_en_map_dir() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("get-proj-dir", r##"Getting the project dir..."##),
    ],
}
}

/// Language ID: en;
/// Map name: "cli";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("cli", "dir");
///
/// assert_eq!(msg, "Location of the directory where the files need to be saved");
/// ```
pub(super) const fn get_en_map_cli() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("dir", r##"Location of the directory where the files need to be saved"##),
    ],
}
}

/// en: English, Latin, United States
pub(super) const fn get_en_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("dir", get_en_map_dir),
        ("cli", get_en_map_cli),
    ],
}
}

/// Language ID: zh;
/// Map name: "dir";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("dir", "get-proj-dir");
///
/// assert_eq!(msg, "正在获取项目文件夹...");
/// ```
pub(super) const fn get_zh_map_dir() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("get-proj-dir", r##"正在获取项目文件夹..."##),
    ],
}
}

/// Language ID: zh;
/// Map name: "cli";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("cli", "dir");
///
/// assert_eq!(msg, "需要保存的文件的目录位置");
/// ```
pub(super) const fn get_zh_map_cli() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("dir", r##"需要保存的文件的目录位置"##),
    ],
}
}

/// zh: 简体中文, 中国
pub(super) const fn get_zh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("dir", get_zh_map_dir),
        ("cli", get_zh_map_cli),
    ],
}
}

/// # Example
///
/// ```no_run
/// let map = locale_map();
///
/// for k in map.keys() {
///     println!("{k}")
/// }
///
/// map.get("en").map(|v| dbg!(v()));
/// ```
pub(super) const fn locale_map() -> LocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("zh", get_zh_map),
        ("en", get_en_map),
    ],
}
}

/// # Example
///
/// ```no_run
/// let loader = glossa::MapLoader::new(locale_hashmap());
///
/// dbg!(&loader);
/// ```
pub(super) fn locale_hashmap() -> LocaleHashMap {
    use lang_id_consts::*;

    HashMap::from_iter([
        (unsafe { get_en() }, get_en_map()),
        (unsafe { get_zh() }, get_zh_map()),
    ])
}
