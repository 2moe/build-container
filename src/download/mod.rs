use error_chain::error_chain;
use std::{fs::File, io::copy, path::Path};

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
pub async fn download_file(target: &str) -> Result<()> {
    let tmp_dir = Path::new("./");
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| {
                if name.is_empty() {
                    None
                } else {
                    Some(name)
                }
            })
            .unwrap_or("images.json");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
