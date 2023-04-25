use crate::assets::locales;

pub(crate) mod src {
    use std::str::FromStr;

    use get_lxc_core::reqwest::Url;

    use crate::url::is_cn;

    /// github
    pub(crate) const GH: &str = "https://api.github.com/repos/2cd/index/tarball/lxc";

    /// 极狐 (gitlab cn)
    pub(crate) const JH: &str =
        "https://jihulab.com/api/v4/projects/106096/repository/archive.tar.gz?sha=lxc";

    /// salsa
    pub(crate) const SA: &str =
    "https://salsa.debian.org/api/v4/projects/70074/repository/archive.tar.gz?sha=lxc";

    // #[allow(dead_code)]
    pub(crate) const TOKEN: &str = "glpat-qRs5xE7DF_FfwDUyd4Ca";

    pub(crate) fn get_src_url(s: &str) -> Url {
        Url::from_str(match s {
            "sa" => SA,
            "gh" => GH,
            "jh" => JH,
            // m if m.starts_with("file://") => m,
            m if m.trim().is_empty() => {
                if is_cn() {
                    JH
                } else {
                    SA
                }
            }
            _ => s,
        })
        .unwrap()
    }
}

pub(crate) fn is_cn() -> bool {
    locales()
        .region
        .map_or(false, |s| s.as_str() == "CN")
}

pub(crate) mod lxc {
    use crate::url::is_cn;
    use get_lxc_core::reqwest::Url;
    use std::str::FromStr;

    pub(crate) const US: &str = "https://us.lxd.images.canonical.com/";
    pub(crate) const UK: &str = "https://uk.lxd.images.canonical.com/";
    pub(crate) const BFSU: &str = "https://mirrors.bfsu.edu.cn/lxc-images/";
    pub(crate) const TUNA: &str = "https://mirrors.tuna.tsinghua.edu.cn/lxc-images/";
    pub(crate) const NJU: &str = "https://mirrors.nju.edu.cn/lxc-images/";
    pub(crate) const ISCAS: &str = "https://mirror.iscas.ac.cn/lxc-images/";

    pub(crate) fn get_lxc_url(s: &str) -> Url {
        Url::from_str(match s {
            "bfsu" => BFSU,
            "tuna" => TUNA,
            "nju" => NJU,
            "us" => US,
            "uk" => UK,
            "iscas" => ISCAS,
            s if s.starts_with("http") => s,
            _ if is_cn() => BFSU,
            _ => US,
        })
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::url::{lxc::*, src::*};
    use get_lxc_core::{
        fetch::{
            dl::Download,
            header::{ff_ua_header, ua_header_map},
        },
        reqwest::header::HeaderValue,
    };

    #[tokio::test]
    async fn get_jh_bz2() {
        let tmp = get_tmp();

        let new = Download::new(JH)
            .unwrap()
            .with_dir(&tmp);

        let _path = new.new_task().await.unwrap();
    }
    fn get_tmp() -> PathBuf {
        envpath::dirs::get_tmp_random_dir(Some(envpath::get_pkg_name!()), None)
    }

    #[tokio::test]
    async fn get_sa_bz2() {
        let tmp = get_tmp();
        let mut header_map = ua_header_map(ff_ua_header());
        header_map.insert("PRIVATE-TOKEN", HeaderValue::from_str(TOKEN).unwrap());

        let new = Download::new(SA)
            .unwrap()
            .with_builder(Download::default_client().default_headers(header_map))
            .with_dir(&tmp);

        let _path = new.new_task().await.unwrap();
    }

    #[tokio::test]
    async fn get_gh_tar_gz() {
        let tmp = get_tmp();

        let new = Download::new(GH)
            .unwrap()
            .with_builder(Download::default_client_with_ff_ua())
            .with_dir(&tmp);

        let _path = new.new_task().await.unwrap();
    }

    #[tokio::test]
    async fn get_nju_lxc_tar_xz() {
        let tmp = get_tmp();

        let new = Download::new(&format!(
            "{BFSU}/images/kali/current/amd64/cloud/20230402_17:14/rootfs.tar.xz"
        ))
        .unwrap()
        .with_builder(Download::default_client_with_ff_ua())
        .with_dir(&tmp);

        let _path = new.new_task().await.unwrap();
    }
}
