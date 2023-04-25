pub(crate) mod async_file;
pub mod dl;
pub mod header;
pub(crate) mod progress_bar;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use reqwest::Client;

    use crate::fetch::{
        dl::Download,
        header::{curl_ua_header, ua_header_map},
    };

    #[tokio::test]
    async fn test_download_file() -> anyhow::Result<()> {
        // let url = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
        let url = "https://mirror.nju.edu.cn/github-release/googlefonts/noto-emoji/LatestRelease/repo-snapshot.tar.gz";
        // let url = "https://gitee.com/mo2/index/repository/archive/lxc.tar.gz";
        // https://gitee.com/api/v5/repos/mo2/linux/archive/master.tar.gz

        let mut map = ua_header_map(curl_ua_header());
        map.insert(
            reqwest::header::AUTHORIZATION,
            "token: 1234567890"
                .parse()
                .unwrap(),
        );

        let path = "a.tar.gz";
        let tmp = std::env::temp_dir();

        let dl = Download::new(url)?
            .with_file_name(Path::new(path))
            .with_dir(&tmp)
            .with_builder(
                Client::builder()
                    .default_headers(map)
                    .gzip(true),
            );
        // dbg!(&dl);

        dl.new_task().await?;
        Ok(())
    }
    #[test]
    fn test_dl_struct() {
        // let a = Url::;
        // dbg!(a);
    }
}
