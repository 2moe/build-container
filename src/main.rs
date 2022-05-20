mod logic;
use logic::{
    download_lxc_toml, parse_lxc_toml, write_url_to_txt,
};

fn main() {
    download_lxc_toml();
    let image_url = parse_lxc_toml();
    write_url_to_txt(&image_url);
}
