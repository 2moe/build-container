use crate::table;
use anyhow::Result;
use comfy_table::{Cell, Color};
use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue, RANGE},
    Client, Url,
};

use std::iter::FromIterator;

// User Agent used for the HTTP request
const FIREFOX_UA: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/109.0";

const CURL_UA: &str = "curl/8.0.1";

pub const fn ff_ua_header() -> [(HeaderName, HeaderValue); 1] {
    [(header::USER_AGENT, HeaderValue::from_static(FIREFOX_UA))]
}

pub const fn curl_ua_header() -> [(HeaderName, HeaderValue); 1] {
    [(header::USER_AGENT, HeaderValue::from_static(CURL_UA))]
}

/// Get the headers for the HTTP request
pub fn ua_header_map(ua: [(HeaderName, HeaderValue); 1]) -> HeaderMap {
    HeaderMap::from_iter(ua)
}

/// Get the size of the file to download and the final URL after redirects
pub(crate) async fn get_header_value(
    client: &Client,
    url: &Url,
    show_header: bool,
) -> Result<(u64, Url, Option<String>)> {
    let resp = client
        .head(url.to_owned()) // Send a HEAD request to the URL to check status
        .send()
        .await?; // Wait for the response

    let url = resp.url().to_owned();
    // Check the HTTP status code and return the download size and final URL
    match resp.error_for_status() {
        Ok(x) => {
            let hmap = x.headers();
            if show_header {
                log::info!("The connection was successful and the Header message is being displayed.");
                let mut table = crate::table::Table::new();
                table::set_header(&mut table, &["Header", "Value"]);

                for (k, v) in hmap {
                    table.add_row([
                        Cell::new(k.as_str()).fg(Color::DarkYellow),
                        Cell::new(v.to_str().unwrap_or("")).fg(Color::DarkCyan),
                    ]);
                }
                println!("{table}\n");
            }

            // content-disposition: attachment; filename="index-lxc-74e1259e7f82a8edfd8a40fdfe7f72c4bc5fea80.tar.bz2"

            // If the response is successful (HTTP 2xx)
            let len = match x.content_length() {
                Some(0) => {
                    // If content length is 0, get the length from headers instead
                    hmap.get(header::CONTENT_LENGTH)
                        .and_then(|x| x.to_str().ok())
                        .and_then(|x| x.parse::<u64>().ok())
                        .unwrap_or(0)
                }
                Some(u) => u, // If content length is not 0, use the provided value
                _ => 0,       // If content length is not provided, assume 0
            };

            // content-disposition: attachment; filename="index-lxc-74e1259e7f82a8edfd8a40fdfe7f72c4bc5fea80.tar.bz2"
            let fname = hmap
                .get(header::CONTENT_DISPOSITION)
                .and_then(|disposition| {
                    disposition
                        .to_str()
                        .ok()
                        .and_then(|x| {
                            x.split(';')
                                .map(|x| x.trim())
                                .filter(|x| x.starts_with("filename"))
                                .find_map(|x| {
                                    x.split_once('=')
                                        .map(|tup| {
                                            tup.1
                                                .trim()
                                                .trim_start_matches('"')
                                                .trim_end_matches('"')
                                                .trim()
                                        })
                                        .filter(|x| !x.is_empty())
                                        .map(ToOwned::to_owned)
                                })
                        })
                });
            Ok((len, url, fname)) // Return the content length and final URL as a tuple
        }
        Err(e) => {
            // If the response is not successful (HTTP 4xx or 5xx)
            match e.status() {
                Some(code) if code == 405 => Ok((0u64, url, None)),
                status => {
                    panic!("Failed to download.\nError: {e}, status: {:?}", status)
                }
            }
        }
    }
}
/// Create an HTTP GET request with appropriate headers and range information
pub(crate) fn create_request_builder(
    client: &Client,
    url: &Url,
    start_byte: u64,
) -> reqwest::RequestBuilder {
    let req = client.get(url.to_owned());
    match start_byte {
        0 => req,
        _ => req.header(RANGE, format!("bytes={}-", start_byte)),
    }
}
