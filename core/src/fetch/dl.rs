use crate::fetch::{
    async_file::{
        create_or_open_file, get_filename, get_local_file_size, AsyncFile,
    },
    header::{
        create_request_builder, curl_ua_header, ff_ua_header, get_header_value,
        ua_header_map,
    },
    progress_bar::create_progress_bar,
};
use anyhow::Result;
use crossterm::style::Stylize;
use getset::{Getters, MutGetters};
use reqwest::{Client, ClientBuilder, Url};
use std::{
    borrow::Cow,
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct Download<'dir, 'fname> {
    dir: Option<&'dir Path>,
    file_name: Option<&'fname Path>,
    builder: Option<ClientBuilder>,
    url: Url,
    header_visibility: bool,
}

impl<'a, 'b> Default for Download<'a, 'b> {
    fn default() -> Self {
        Self {
            dir: None,
            file_name: None,
            builder: None,
            url: "http://127.0.0.1"
                .parse()
                .expect("Failed to set default url"),
            header_visibility: true,
        }
    }
}

impl<'dir, 'fname> Download<'dir, 'fname> {
    pub fn new(url: &str) -> Result<Self, <Url as FromStr>::Err> {
        Ok(Self {
            url: {
                match url {
                    u if u.contains("://") => url.parse()?,
                    u => format!("https://{u}").parse()?,
                }
            },
            ..Default::default()
        })
    }

    pub fn new_with_url(url: Url) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }

    pub fn with_file_name(self, fname: &'fname Path) -> Self {
        Self {
            file_name: Some(fname),
            ..self
        }
    }

    pub fn with_dir(self, dir: &'dir Path) -> Self {
        Self {
            dir: Some(dir),
            ..self
        }
    }

    pub fn with_builder(self, builder: ClientBuilder) -> Self {
        Self {
            builder: Some(builder),
            ..self
        }
    }

    pub fn switch_header_visibility(self, switch: bool) -> Self {
        Self {
            header_visibility: switch,
            ..self
        }
    }

    pub fn default_client_with_ff_ua() -> ClientBuilder {
        Client::builder()
            .default_headers(ua_header_map(ff_ua_header()))
            .gzip(true)
    }

    pub fn default_client_with_curl_ua() -> ClientBuilder {
        Client::builder()
            .default_headers(ua_header_map(curl_ua_header()))
            .gzip(true)
    }

    pub fn default_client() -> ClientBuilder {
        Client::builder().gzip(true)
    }

    pub async fn new_task(self) -> Result<PathBuf> {
        // Create a new HTTP client with default headers and gzip enabled
        let client = match self.builder {
            Some(x) => x,
            _ => Self::default_client_with_ff_ua(),
        }
        .build()?;

        // Get the content-length and the final URL after redirects
        let (full_size, redirect_url, opt_fname) =
            get_header_value(&client, &self.url, true).await?;

        // Get the filename from the final URL and sanitize it
        let file_name = match self.file_name {
            Some(x) => Cow::from(x),
            _ => match opt_fname {
                Some(s) => Cow::Owned(PathBuf::from(s)),
                _ => get_filename(&redirect_url),
            },
        };

        let path = match self.dir {
            Some(x) => {
                std::fs::create_dir_all(x)?;
                x.join(&file_name)
            }
            _ => env::current_dir()
                .unwrap_or(PathBuf::default())
                .join(&file_name),
        };

        println!("{}:\t{}", "File".dark_blue(), path.display());

        // Get the local file size to determine where to resume downloading
        let local_size = get_local_file_size(&path)?;

        // Create an HTTP GET request with appropriate headers and range information
        let request = create_request_builder(&client, &redirect_url, local_size);

        // Send the HTTP request and get a response stream
        let mut dl_resp = request.send().await?;

        let mut out = {
            let outfile = create_or_open_file(&path, local_size, full_size).await?;
            AsyncFile::new(full_size, outfile)
        };

        // Create a progress bar
        let bar = create_progress_bar(full_size, &file_name, local_size)?;

        // Loop through each chunk in the response stream
        while let Some(chunk) = dl_resp.chunk().await? {
            // Update the progress bar and write the chunk to the output file
            bar.inc(chunk.len() as u64);
            out.write_all(&chunk).await?;
        }

        out.flush().await?;

        if full_size == 0 {
            let size = get_local_file_size(&path)?;
            bar.finish_and_clear();
            bar.set_length(size);
            bar.set_position(size);
        }

        // Finish the progress bar and flush the output file buffer
        bar.finish_with_message(
            "Download complete"
                .dark_green()
                .italic()
                .to_string(),
        );

        Ok(path)
    }
}
