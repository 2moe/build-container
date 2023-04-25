pub(crate) mod localisation;
use glossa::{
    assets::{lang_id_consts, HashMap, OnceCell},
    LangID, MapLoader,
};

use crate::assets::localisation::locale_hashmap;

pub(crate) fn locales() -> &'static MapLoader {
    static RES: OnceCell<MapLoader> = OnceCell::new();
    RES.get_or_init(|| MapLoader::new(locale_hashmap()))
}
