mod assets;
mod cfg;
mod cli;
mod error;
mod logger;
pub(crate) mod url;
use crate::{
    cfg::core::dir::get_static_data_dir, cli::parser::parse_args,
    logger::set_date_arr_and_logger,
};
use anyhow::Result;
mod archive;

fn main() -> Result<()> {
    let data_dir = get_static_data_dir();
    set_date_arr_and_logger(data_dir.get_data())?;

    parse_args()?;

    Ok(())
}

#[test]
fn get_os_arch() {
    eprintln!("{}", std::env::consts::ARCH)
}
