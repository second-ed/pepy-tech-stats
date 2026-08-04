pub mod adapters;
pub mod domain;

use crate::core::{
    adapters::{Adapter, IoValue, ParamKey, ParamValue},
    domain::{
        errors::PepyStatsError,
        extract_project_stats::{process_project_stats, REQUESTS_PER_MIN},
        update_readme::update_readme,
    },
};
use chrono::{Duration, Utc};
use flexi_logger::{Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming};
use log;
use serde_json::Value;
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

    let yesterday = (Utc::now().date_naive() - Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();

    let _readme_table = process_project_stats(adapter, projects, REQUESTS_PER_MIN)
        .map(|values| parse_package_stats(&values, &yesterday))
        .map(package_stats_to_readme_table)
        .and_then(|readme_table| update_readme(adapter, readme_table, readme_path));

    Ok(RetCode::OK)
}

fn parse_package_stats(values: &[IoValue], yesterday: &String) -> Vec<PackageStats> {
    values
        .iter()
        .map(|v| match v {
            IoValue::Json(j) => j,
            IoValue::Str(_) => unreachable!(),
        })
        .map(|value| PackageStats::from_request(value, yesterday))
        .collect::<Vec<PackageStats>>()
}

fn package_stats_to_readme_table(mut package_stats: Vec<PackageStats>) -> ReadMeTable {
    package_stats.sort_by_key(|p| -p.total_downloads);

    let total_downloads: i64 = package_stats
        .iter()
        .map(|package| package.total_downloads)
        .sum();

    let yesterday_downloads: i64 = package_stats
        .iter()
        .map(|package| package.yesterday_downloads)
        .sum();

    let mut lines = vec![
        format!("total downloads: `{}`\n", total_downloads),
        format!("yesterday downloads: `{}`\n", yesterday_downloads),
        "### breakdown by package".to_string(),
        "| package | total_downloads | yesterday_downloads |".to_string(),
        "| --- | --- | --- |".to_string(),
    ];

    lines.extend(package_stats.iter().map(PackageStats::table_line));
    ReadMeTable::new(lines)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct PackageStats {
    package: String,
    total_downloads: i64,
    yesterday_downloads: i64,
}

impl PackageStats {
    fn new(package: String, total_downloads: i64, yesterday_downloads: i64) -> Self {
        Self {
            package,
            total_downloads,
            yesterday_downloads,
        }
    }

    fn from_request(response: &serde_json::Value, yesterday: &String) -> Self {
        Self::new(
            response
                .get("id")
                .and_then(|name| name.as_str())
                .unwrap_or_default()
                .to_string(),
            response
                .get("total_downloads")
                .unwrap_or_default()
                .as_i64()
                .unwrap_or_default(),
            response
                .get("downloads")
                .unwrap_or_default()
                .get(yesterday)
                .unwrap_or_default()
                .as_object()
                .map(|versions| versions.values().filter_map(Value::as_i64).sum())
                .unwrap_or_default(),
        )
    }

    fn table_line(&self) -> String {
        format!(
            "| {} | {} | {} |",
            self.package, self.total_downloads, self.yesterday_downloads
        )
    }
}

#[derive(Debug)]
pub struct ReadMeTable {
    lines: Vec<String>,
}

impl ReadMeTable {
    #[must_use]
    pub const fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.lines.join("\n")
    }
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
                &record.args()
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
