use flexi_logger::{
    detailed_format, style, Duplicate, FileSpec, FlexiLoggerError, Logger,
    LoggerHandle,
};
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

use crate::cfg::core::get_static_core_cfg;

type LoggerResult = Result<LoggerHandle, FlexiLoggerError>;

pub(crate) fn set_date_arr_and_logger(data_dir: &Path) -> LoggerResult {
    const DATE_ARR_LEN: usize = 3;
    let today = get_today_str();
    let mut date = [""; DATE_ARR_LEN];
    for (i, s) in today
        .split('-')
        .take(DATE_ARR_LEN)
        .enumerate()
    {
        unsafe { *date.get_unchecked_mut(i) = s }
    }
    init_logger(data_dir, &date)
}

fn get_today_str() -> String {
    get_local_time()
        .date()
        .to_string()
}

fn get_local_time() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

fn get_level(lv: &str) -> (Duplicate, &str) {
    use Duplicate::*;
    match lv {
        "err" | "error" | "Error" => (Error, "error"),
        "warn" | "Warn" | "warning" => (Warn, "warn"),
        "info" | "Info" => (Info, "info"),
        "dbg" | "debug" | "Debug" => (Debug, "debug"),
        "trace" | "all" | "Trace" => (Trace, "trace"),
        _ => (None, ""),
    }
}

fn init_logger(data_dir: &Path, date: &[&str]) -> LoggerResult {
    const LOG_TIME_FMT: &str = "%H:%M:%S%.3f";

    let cfg = get_static_core_cfg().get_logging();

    let file_lv = get_level(cfg.get_file_level()).1;

    let stderr_lv = get_level(cfg.get_stderr_level()).0;
    Logger::try_with_env_or_str(file_lv)?
        .log_to_file(
            FileSpec::default()
                .basename(date[2])
                .directory(
                    data_dir.join(PathBuf::from_iter(["logs", date[0], date[1]])),
                )
                .suppress_timestamp(),
        )
        .duplicate_to_stderr(stderr_lv)
        .append()
        .format_for_files(detailed_format)
        .format_for_stderr(|w, now, record| {
            let level = record.level();
            write!(
                w,
                "{} {} [{}] {}",
                now.format(LOG_TIME_FMT),
                style(level).paint(level.to_string()),
                record
                    .module_path()
                    .unwrap_or(""),
                style(level).paint(record.args().to_string())
            )
        })
        .o_print_message(
            cfg.get_show_location()
                .to_owned(),
        )
        .start()
}
