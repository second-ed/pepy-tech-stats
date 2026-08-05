pub mod adapters;
pub mod domain;

use crate::core::{
    adapters::{Adapter, ParamKey, ParamValue},
    domain::{
        errors::PepyStatsError,
        extract_project_stats::{process_project_stats, REQUESTS_PER_MIN},
        transform::{package_stats_to_readme_table, parse_package_stats, yesterday},
        update_readme::update_readme,
    },
};
use flexi_logger::{Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming};
use log;
use std::io::Write;

pub enum RetCode {
    OK,
    ERR,
}

pub fn run(
    adapter: &mut impl Adapter,
    projects: &[String],
    api_key: String,
) -> Result<RetCode, PepyStatsError> {
    let _ = configure_logger();
    log::info!("Starting process for projects: {projects:?}");
    adapter.add_param(ParamKey::ApiKey, ParamValue::Str(api_key));

    let readme_path = "./README.md";
    let yesterday = yesterday();

    let _ = process_project_stats(adapter, projects, REQUESTS_PER_MIN)
        .map(|values| parse_package_stats(&values, &yesterday))
        .map(package_stats_to_readme_table)
        .and_then(|readme_table| update_readme(adapter, readme_table, readme_path));

    Ok(RetCode::OK)
}

pub fn configure_logger(
) -> std::result::Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    Logger::try_with_env_or_str("info")?
        .format(|w: &mut dyn Write, now: &mut DeferredNow, record| {
            write!(
                w,
                "{} [{} | {} | {}] - {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .duplicate_to_stdout(Duplicate::All)
        .log_to_file(FileSpec::default().directory("logs").basename("app"))
        .rotate(
            Criterion::Size(2_000_000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .start()
}
