use get_arch_url::{
    download::download_file, toml_parser::doc::Doc,
};
use lazy_static::lazy_static;
use nom::{
    bytes::complete::{tag, take_until1},
    sequence::delimited,
};
use std::env;

lazy_static! {
    static ref ARCH: String = env::var("ARCH").unwrap();
}

pub(crate) fn download_lxc_toml() {
    let url = "https://github.com/2cd/index/raw/master/containers/lxc/lxc.toml";
    download_file(url).unwrap();
}

pub(crate) fn write_url_to_txt(url: &str) {
    println!("{}", url);
    let new_contents = url.as_bytes();

    if let Err(e) = std::fs::write("url.txt", new_contents)
    {
        eprintln!("{}", e)
    }
}

fn cut_quote(s: &str) -> nom::IResult<&str, &str> {
    delimited(tag("\""), take_until1("\""), tag("\""))(s)
}

const LXC_URL: &str = "https://images.linuxcontainers.org";

pub(crate) fn parse_lxc_toml() -> String {
    let doc = Doc::from_file("lxc.toml");

    let r = doc.doc.as_table();

    let table = r["products"]["archlinux"]["current"]
        [&*ARCH]["default"]
        .as_table()
        .unwrap();
    let products = table["time"]
        .as_array()
        .unwrap();
    let value = products
        .get(0)
        .unwrap()
        .to_string();
    let (_, value) = cut_quote(&value).unwrap();

    let lxc_path = table[value]
        .as_table()
        .unwrap();
    let vv = lxc_path["path"]
        .as_array()
        .unwrap()
        .get(0)
        .unwrap()
        .to_string();

    let (_, bb) = cut_quote(&vv).unwrap();
    [LXC_URL, bb, "rootfs.tar.xz"].concat()
}
