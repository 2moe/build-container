use clap::{Parser, Subcommand};
use getset::Getters;
use glossa::GetText;
use std::path::PathBuf;

use crate::assets::locales;

fn get_text<'key: 'res, 'res>(
    map_name: &'key str,
    key: &'key str,
) -> Option<&'res str> {
    locales()
        .get(map_name, key)
        .ok()
}
// fn version() -> &'static str {
//     "1.0.1"
// }
/// A test cli
#[derive(Parser, Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
// #[command(max_term_width = 80)]
// #[command(disable_help_flag = true)]
#[command(version, arg_required_else_help = true)]
#[command(long_version = "commit: 1.0
last-update: 2022-12-21
")]
pub(crate) struct Cli {
    #[arg(
        short,
        long,
        value_name = "arch, deb, etc.",
        num_args = 0..=1,
        default_missing_value = "",
    )]
    os: Option<String>,

    /// Set the codename.
    ///
    /// For example, if the OS is debian, then you can type the codename: "bookworm"
    #[arg(
        short,
        long = "code",
        id = "code_name",
        alias = "codename",
        num_args = 0..=1,
        default_missing_value = "",
        requires = "os",
    )]
    codename: Option<String>,

    #[arg(
        long = "var",
        id = "var",
        value_name = "cloud, default, etc.",
        num_args = 0..=1,
        alias = "variant",
        // default_value = variant_key_default().0,
        default_missing_value = "",
        requires_all(["os", "code_name"])
    )]
    variant: Option<String>,

    #[arg(
        short,
        long,
        value_name = "amd64, riscv64, arm64, etc.",
        // default_value = arch_key_default().0,
        default_missing_value = "",
        num_args = 0..=1,
        // requires_all(["os", "code_name", "var"])
    )]
    arch: Option<String>,

    #[arg(
        short,
        long,
        value_name = "0, 1, 2, or tag_name",
        num_args = 0..=1,
        default_missing_value = ""
    )]
    tag: Option<String>,

    // /// Load config file. For windows: 'C:\path\to\your\file', Unix: "/path/to/file"
    // #[arg(short, long = "cfg", id = "config_file")]
    // config: Option<String>,
    //
    /// A config can have multi profiles.
    #[arg(help = get_text("cli", "dir"))]
    // #[arg(short, long, id = "profile_name")]
    #[cfg(not(target_arch = "wasm32"))]
    #[arg(short, long, value_name = sample_directory(true), value_hint = clap::ValueHint::DirPath)]
    dir: Option<PathBuf>,

    /// e.g. uuu.tar.xz
    #[arg(short, long = "fname", alias = "filename", id = "file_name", value_hint = clap::ValueHint::FilePath)]
    filename: Option<PathBuf>,

    /// Skip verify the file digest
    // #[arg(long)]
    // skip_verify: bool,

    /// Set the style of the table
    // #[arg(short, long, value_name = "raw, def")]
    // style: Option<String>,

    #[arg(short, long, value_name = "us, uk, nju, bfsu, tuna", value_hint = clap::ValueHint::Url)]
    mirror: Option<String>,

    //or https url
    #[arg(long, value_name = "gh, jh, sa", value_hint = clap::ValueHint::Url)]
    src: Option<String>,

    // #[arg(long, help = version())]
    // version: bool,
    #[command(subcommand)]
    cfg: Option<Config>,
    // #[arg(index = 0, hide= true)]
    // #[arg(short = '?', hide= true, action = ArgAction::Help)]
    // h: bool,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Config {
    /// Load config
    #[command(arg_required_else_help = true)]
    #[command(name = "config", visible_aliases = ["c","cfg"])]
    Cfg {
        #[arg(short, long, value_name = sample_directory(false))]
        file: Option<PathBuf>,

        #[arg(
            short, long, num_args = 0..=1,
            value_name = "profile_name",
            default_missing_value = ""
        )]
        profile: Option<String>,

        // `-g a.b.c.d`
        // #[arg(
        //     short, long, num_args = 0..=1,
        //     value_name = "key",
        //     default_missing_value = ""
        // )]
        // get: Option<String>,

        // /// `-s "a.b.c" "val"`
        // #[arg(
        //     short, long,
        //     num_args = 1..=2,
        //     value_names = ["key","value"]
        // )]
        // set: Option<Vec<String>>,

        // #[arg(short, long)]
        // dry_run: bool,
        /// reset to default config
        #[arg(long)]
        reset: bool,
    },
    #[command(arg_required_else_help = true)]
    #[command(visible_aliases = ["u", "upd"])]
    Update {
        /// Update self, index
        #[arg(value_name = "index", index = 1)]
        name: Option<String>,
    },
}

const fn sample_directory(b: bool) -> &'static str {
    #[cfg(not(windows))]
    match b {
        true => "/path/to/directory",
        _ => "/path/to/file",
    }

    #[cfg(windows)]
    match b {
        true => r"C:\path\to\directory",
        _ => r"C:\path\to\file",
    }
}
