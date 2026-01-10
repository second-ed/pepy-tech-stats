pub mod adapters;
pub mod domain;

use crate::core::adapters::{Adapter, FileType, ParamKey, ParamValue};
use crate::core::domain::extract_project_stats::{process_project_stats, REQUESTS_PER_MIN};
use crate::core::domain::transform::{responses_to_df, transform_dataframe};
use flexi_logger::DeferredNow;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use log;
use std::io::Write;
use std::path::{Path, PathBuf};

pub enum RetCode {
    OK,
    ERR,
}

pub fn main(
    adapter: &mut impl Adapter,
    projects: Vec<String>,
    api_key: String,
) -> Result<RetCode, RetCode> {
    let _ = configure_logger();
    log::info!("Starting process for projects: {:?}", projects);
    adapter.add_param(ParamKey::ApiKey, ParamValue::Str(api_key));

    // let transformed_df = process_project_stats(adapter, projects, REQUESTS_PER_MIN)
    //     .and_then(responses_to_df)
    //     .and_then(transform_dataframe);

    // let _ = dbg!(transformed_df);

    let res = adapter.read(&PathBuf::from("./README.md"), FileType::Str);

    let _ = dbg!(res);

    Ok(RetCode::OK)
}

pub fn configure_logger(
) -> std::result::Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    Logger::try_with_env_or_str("info")
        .unwrap()
        .format(|w: &mut dyn Write, now: &mut DeferredNow, record| {
            write!(
                w,
                "{} [{} | {} | {} | {}] - {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                &record.args()
            )
        })
        .log_to_stdout()
        .log_to_file(FileSpec::default().directory("logs").basename("app"))
        .rotate(
            Criterion::Size(2000000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .start()
}
