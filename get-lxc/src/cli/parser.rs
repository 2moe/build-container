use crate::{
    archive::gzip::extract_specific_level_tgz,
    cfg::{
        core::{
            dir::{get_static_cfg_dir, get_static_data_dir},
            CoreCfg,
        },
        get_cfg,
        index::LxcIndex,
        user::{get_static_user_cfg, ProfileAttr, UserCfg},
    },
    cli::opt::{Cli, Config},
    url::{
        lxc::get_lxc_url,
        src::{get_src_url, TOKEN},
    },
};
use anyhow::Result;
use clap::Parser;
use crossterm::style::Stylize;
use get_lxc_core::{
    as_tiny_str,
    cfg::{ArchKey, CodeKey, OsName, RootFS, TagKey, VariantKey},
    fetch::{
        dl::Download,
        header::{ff_ua_header, ua_header_map},
    },
    reqwest::{header::HeaderValue, Url},
    table::{self, Table},
};
use glossa::log::*;
use std::{
    borrow::Cow,
    ffi::OsStr,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};
use time::OffsetDateTime;

#[tokio::main]
pub(crate) async fn parse_args() -> Result<()> {
    let args = Cli::parse();

    let (cfg, profile) = match args.get_cfg() {
        Some(s) => handle_subcmd(s)?,
        _ => (None, Cow::from("default")),
    };

    let user_cfg = cfg
        .as_ref()
        .unwrap_or_else(|| get_static_user_cfg());

    let profile_attr = user_cfg
        .get_profile()
        .get(profile.as_ref());

    let (src, mirror) = get_src_and_mirror(profile_attr, &args);
    let (need_to_update, mut lxc_cfg) = check_lxc_index();

    if need_to_update {
        get_lxc_index(src).await?;
        lxc_cfg = check_lxc_index().1;
    }

    let img_prof = profile_attr.and_then(|p| p.get_img().as_ref());
    let os_prof = img_prof.and_then(|p| p.get_os().as_ref());

    let print_os_table = || {
        let mut table = Table::new();
        table::set_header(&mut table, &["Operating System"]);
        for r in lxc_cfg.get_list() {
            table.add_row([r]);
        }
        println!("{table}");
        exit(0)
    };

    let org_os = match args.get_os() {
        Some(os) if os.trim().is_empty() => print_os_table(),
        Some(os) => os,
        _ => os_prof.unwrap_or_else(print_os_table),
    };

    let is_std_os_name = lxc_cfg
        .get_list()
        .contains(org_os);

    let os_name = match is_std_os_name {
        true => OsName::from_str(org_os)?,
        _ => {
            match user_cfg
                .get_os()
                .get_alias()
                .get(org_os)
            {
                Some(o) => o.to_owned(),
                _ => {
                    err!("Invalid OS Alias");
                    let mut table = Table::new();
                    table::set_header(&mut table, &["alias", "value"]);
                    let map = user_cfg.get_os().get_alias();
                    for (k, v) in map {
                        table.add_row([k, v.as_str()]);
                    }
                    println!("{table}");
                    exit(0)
                }
            }
        }
    };
    // ----
    // get ron cfg
    let ron_cfg_file = get_static_data_dir()
        .get_cache()
        .iter()
        .chain([
            OsStr::new("index"),
            Path::new(os_name.as_str())
                .with_extension("ron")
                .as_os_str(),
        ])
        .collect::<PathBuf>();

    let ron_cfg = crate::cfg::de::get_lxc_root_cfg(ron_cfg_file)?;
    // codename:
    // Parsing RON configuration
    let print_code_table = || {
        let mut table = Table::new();
        table::set_header(&mut table, &["CodeName", "Variant", "Architecture"]);

        for (k, v) in ron_cfg.get_codename() {
            for (variant, architecture) in v.get_variant() {
                for arch in architecture.get_arch().keys() {
                    table.add_row([k.as_str(), variant.as_str(), arch.as_str()]);
                }
            }
        }
        println!("{table}");
        exit(0)
    };

    let code_prof = img_prof.and_then(|p| p.get_codename().as_ref());

    let org_code = match args.get_codename() {
        Some(os) if os.trim().is_empty() => print_code_table(),
        Some(os) => os,
        _ => code_prof.unwrap_or_else(print_code_table),
    };

    let code_key = CodeKey::from_str(org_code)?;

    let is_std_code_name = ron_cfg
        .get_codename()
        .contains_key(&code_key);

    let code_name = match is_std_code_name {
        true => code_key,
        _ => {
            match user_cfg
                .get_codename()
                .get_alias()
                .get(org_code)
            {
                Some(o) => o.to_owned(),
                _ => {
                    err!("Invalid CodeName Alias");
                    let mut table = Table::new();
                    table::set_header(&mut table, &["alias", "value"]);
                    let map = user_cfg
                        .get_codename()
                        .get_alias();
                    for (k, v) in map {
                        table.add_row([k, v.as_str()]);
                    }
                    println!("{table}");
                    exit(0)
                }
            }
        }
    };

    // variant
    let variant_map = ron_cfg
        .get_codename()
        .get(&code_name)
        .expect("Invalid Variant Map")
        .get_variant();

    let print_variant_table = || {
        let mut table = Table::new();
        table::set_header(&mut table, &["Variant"]);
        for k in variant_map.keys() {
            table.add_row([k.as_str()]);
        }
        println!("{table}");
        exit(0)
    };

    let variant_prof = img_prof.and_then(|p| p.get_variant().as_ref());

    let org_variant = match args.get_variant() {
        Some(os) if os.trim().is_empty() => print_variant_table(),
        Some(os) => os,
        _ => variant_prof.unwrap_or_else(print_variant_table),
    };

    let var_key = VariantKey::from_str(org_variant)?;

    let is_std_var_name = variant_map.contains_key(&var_key);

    let variant_name = match is_std_var_name {
        true => var_key,
        _ => {
            match user_cfg
                .get_variant()
                .get_alias()
                .get(org_variant)
            {
                Some(o) => as_tiny_str(o.as_str()),
                _ => {
                    err!("Invalid Variant Name Alias");
                    let mut table = Table::new();
                    table::set_header(&mut table, &["alias", "value"]);
                    let map = user_cfg
                        .get_variant()
                        .get_alias();
                    for (k, v) in map {
                        table.add_row([k, v.as_str()]);
                    }
                    println!("{table}");
                    exit(0)
                }
            }
        }
    };

    info!("codename: {}", variant_name);

    // arch
    let tag_map = variant_map
        .get(&variant_name)
        .expect("Invalid Architecture Map")
        .get_arch();

    let print_arch_table = || {
        let mut table = Table::new();
        table::set_header(&mut table, &["Architecture"]);
        for k in tag_map.keys() {
            table.add_row([k.as_str()]);
        }
        println!("{table}");
        exit(0)
    };

    let arch_prof = img_prof.and_then(|p| p.get_arch().as_ref());

    let org_arch = match args.get_arch() {
        Some(os) if os.trim().is_empty() => print_arch_table(),
        Some(os) => os,
        _ => arch_prof.unwrap_or_else(print_arch_table),
    };

    let arch_key = ArchKey::from_str(org_arch)?;

    let is_std_arch_name = tag_map.contains_key(&arch_key);

    let arch_name = match is_std_arch_name {
        true => arch_key,
        _ => {
            match user_cfg
                .get_arch()
                .get_alias()
                .get(org_arch)
            {
                Some(o) => as_tiny_str(o.as_str()),
                _ => {
                    err!("Invalid Architecture");
                    print_arch_table();
                    exit(0)
                }
            }
        }
    };
    info!("Architecture: {}", arch_name);

    // tag
    let tag_map = tag_map
        .get(&arch_name)
        .expect("Invalid tag Map")
        .get_tags();

    let print_tag_table = || -> ! {
        let mut table = Table::new();
        table::set_header(&mut table, &["Tag", "Full Tag"]);
        for (i, k) in tag_map.keys().enumerate() {
            table.add_row([&format!("{i}"), k.as_str()]);
        }
        println!("{table}");
        exit(0)
    };

    let tag_prof = img_prof.and_then(|p| p.get_tag().as_ref());

    let tag_key = match args.get_tag() {
        Some(t) if t.trim().is_empty() => print_tag_table(),
        Some(t) => match t.parse::<usize>() {
            Ok(u) => tag_map
                .keys()
                .nth(u)
                .unwrap_or_else(|| print_tag_table())
                .to_owned(),
            _ => TagKey::from_str(t)?,
        },
        _ => TagKey::from_str(tag_prof.unwrap_or_else(|| print_tag_table()))?,
    };

    // let tag_key = TagKey::from_str(org_tag)?;
    let is_std_tag_name = tag_map.contains_key(&tag_key);

    let tag_name = match is_std_tag_name {
        true => tag_key,
        _ => {
            err!("Invalid Tag");
            print_tag_table();
        }
    };
    info!("Tag: {}", tag_name);

    // Download files, if --dir and --fname are not specified, use the default directory
    // check cli first, then use profile, and finally use core cfg
    let path_map = tag_map
        .get(&tag_name)
        .expect("Invalid tag Map");

    get_lxc_img_file(path_map, mirror, &args).await?;
    Ok(())
}

async fn get_lxc_img_file(map: &RootFS, mirror: Url, args: &Cli) -> Result<()> {
    // todo!()
    let path = map.get_path();
    println!(
        "digest: {}, file-size: {}",
        map.get_digest().get_hex(),
        map.get_size().get_str()
    );
    dbg!(&path);

    // if !mirror.as_str().ends_with('/') {}
    let url = match mirror.as_str() {
        s if !s.ends_with('/') => Url::from_str(&format!("{s}/{path}"))?,
        _ => mirror.join(path)?,
    };

    // println!("{url}");
    // exit(0);

    let static_dir = get_static_data_dir().get_dl();

    let mut dl = Download::new_with_url(url);

    match (args.get_dir(), args.get_filename()) {
        (None, None) => *dl.get_dir_mut() = Some(static_dir),
        (None, Some(f)) => *dl.get_file_name_mut() = Some(f),
        (Some(d), None) => *dl.get_dir_mut() = Some(d),
        (Some(d), Some(f)) => {
            *dl.get_dir_mut() = Some(d);
            *dl.get_file_name_mut() = Some(f);
        }
    };

    // let _dl_file =
    dl.new_task().await?;
    Ok(())
}

fn get_src_and_mirror(prof: Option<&ProfileAttr>, args: &Cli) -> (Url, Url) {
    let src = match args.get_src() {
        Some(m) => get_src_url(m),
        _ => prof
            .and_then(|r| r.get_src().as_deref())
            .map(get_src_url)
            .unwrap(),
    };

    let mirror = match args.get_mirror() {
        Some(m) => get_lxc_url(m),
        _ => prof
            .and_then(|r| r.get_mirror().as_deref())
            .map(get_lxc_url)
            .unwrap(),
    };
    (src, mirror)
}

fn check_lxc_index() -> (bool, LxcIndex) {
    let cache_dir = get_static_data_dir().get_cache();
    let main_index = cache_dir
        .iter()
        .chain(["index", "main.toml"].map(OsStr::new))
        .collect::<PathBuf>();

    let index_cfg = crate::cfg::get_cfg::<LxcIndex>("", Some(&main_index));

    let last_update = index_cfg
        .get_time()
        .get_last_update();
    let now = OffsetDateTime::now_utc();
    let need_to_update =
        last_update.saturating_add(time::Duration::hours(12)) <= now;

    (need_to_update, index_cfg)
}

async fn get_lxc_index(src: Url) -> Result<()> {
    let mut dl =
        Download::new_with_url(src).with_dir(get_static_data_dir().get_cache());

    if dl
        .get_url()
        .as_str()
        .contains("salsa.debian.org")
    {
        let mut header_map = ua_header_map(ff_ua_header());
        header_map.insert("PRIVATE-TOKEN", HeaderValue::from_str(TOKEN)?);

        *dl.get_builder_mut() =
            Some(Download::default_client().default_headers(header_map));
    }
    println!("Getting the index");

    let dl_file = dl.new_task().await?;

    extract_specific_level_tgz(&dl_file, None, Some("index"), 2, Some(5))?;

    fs::remove_file(&dl_file)?;
    Ok(())
}

fn handle_subcmd(cli_cfg: &Config) -> Result<(Option<UserCfg>, Cow<str>)> {
    if let Config::Update { name } = cli_cfg {
        dbg!(&name);
        panic!("This feature is under development...");
        // exit(0)
    }

    let Config::Cfg {
        file,
        profile,
        reset} = cli_cfg  else {
            panic!("Failed to get subcommand args")
    };

    let cfg_toml = get_static_cfg_dir().join("cfg.toml");
    let core_toml = get_static_cfg_dir().join("core.toml");

    let prof = profile
        .as_ref()
        .map_or_else(|| Cow::from("default"), Cow::from);

    let opt_cfg = match file {
        Some(f) if !f.is_dir() => Some(get_cfg::<UserCfg>("", Some(f))),
        _ => None,
    };
    if *reset {
        reset_to_default(&core_toml, &cfg_toml)
    }

    Ok((opt_cfg, prof))
}

fn reset_to_default(core_toml: &Path, cfg_toml: &Path) {
    warning!("This action will reset to the default profile, are you sure you want to do this?{}", "[Y/n]".green());

    let path_to_str = |p: &Path| p.display().to_string();

    warning!(
        "\ncore-config: {}\nuser-config: {}",
        path_to_str(core_toml).dark_blue(),
        path_to_str(cfg_toml).dark_cyan(),
    );

    let mut byte = [0u8];

    let rename = |src, dst| fs::rename(src, dst).expect("Unable to rename file");

    match io::stdin().read_exact(&mut byte) {
        Ok(_) if matches!(&byte, b"Y" | b"y" | b"\n" | b"\r") => {
            let new_cfg = get_cfg::<UserCfg>;

            for (src, dst) in [core_toml, cfg_toml]
                .iter()
                .zip([core_toml, cfg_toml].map(|p| p.with_extension("bak")))
            {
                rename(src, dst)
            }
            new_cfg("cfg.toml", None);
            get_cfg::<CoreCfg>("core.toml", None);
        }
        Ok(_) => {
            warning!("Skip");
        }
        Err(e) => {
            err!("{e}");
            panic!("");
        }
    }
    exit(0)
}
