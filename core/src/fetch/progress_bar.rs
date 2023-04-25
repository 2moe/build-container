use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tokio::time::Duration;

// Progress bar template
const PROGRESS_TMPL: &str = "\n{bytes_per_sec:<11.yellow}\n{wide_bar:<.green/black} {bytes:>.green}/{total_bytes:<.cyan} \n{msg}\t{percent:.blue}%\t time {elapsed}";

// const PROGRESS_TMPL_ZERO: &str = "\n{wide_bar:<.green/black} {bytes:>.green}/{total_bytes:<.cyan} \n{msg}\t{percent:.blue}%\t time {elapsed}";
const PROGRSS_CHARS: &str = "━╾╴─";

/// Create a progress bar
pub(crate) fn create_progress_bar<P: AsRef<Path>>(
    full_size: u64,
    file_name: P,
    start_byte: u64,
) -> Result<ProgressBar> {
    let fname = file_name.as_ref();

    let bar = ProgressBar::new(full_size);

    // Set the progress bar style and initial values
    bar.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_TMPL)?
            .progress_chars(PROGRSS_CHARS),
    );

    bar.set_position(start_byte);
    // bar.inc_length(start_byte);

    bar.set_message(format!("Downloading {:.20} ...", fname.display()));
    bar.enable_steady_tick(Duration::from_millis(100));

    Ok(bar)
}
