pub mod adapters;
pub mod domain;

use crate::core::{
    adapters::{Adapter, IoValue, ParamKey, ParamValue},
    domain::{
        errors::PepyStatsError,
        extract_project_stats::{process_project_stats, REQUESTS_PER_MIN},
        transform::{df_to_md, responses_to_df, transform_dataframe},
        update_readme::update_readme,
    },
};
use chrono::{Duration, Utc};
use flexi_logger::{Cleanup, Criterion, DeferredNow, FileSpec, Logger, Naming};
use log;
use serde_json::json;
use std::io::Write;

pub enum RetCode {
    OK,
    ERR,
}

pub fn main(
    adapter: &mut impl Adapter,
    projects: Vec<String>,
    api_key: String,
) -> Result<RetCode, PepyStatsError> {
    let _ = configure_logger();
    log::info!("Starting process for projects: {:?}", projects);
    adapter.add_param(ParamKey::ApiKey, ParamValue::Str(api_key));

    // SCRAP

    fn mock_responses() -> Result<Vec<IoValue>, PepyStatsError> {
        let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();

        Ok(vec![
            IoValue::Json(json!({
                "id": "some_package",
                "total_downloads": 100,
                "versions": ["0.1.0", "0.2.0"],
                "downloads": {
                    yesterday.clone(): {
                        "0.1.0": 10,
                        "0.2.0": 10
                    },
                    "2026-01-01": {
                        "0.1.0": 5,
                        "0.2.0": 20
                    },
                },
            })),
            IoValue::Json(json!({
                "id": "some_other_package",
                "total_downloads": 200,
                "versions": ["0.1.0", "0.2.0"],
                "downloads": {
                    yesterday.clone(): {
                        "0.1.0": 30,
                        "0.2.0": 30
                    },
                    "2026-01-01": {
                        "0.1.0": 5,
                        "0.2.0": 20
                    },
                },
            })),
        ])
    }

    // let readme_table = mock_responses()
    let readme_table = process_project_stats(adapter, projects, REQUESTS_PER_MIN)
        .and_then(responses_to_df)
        .and_then(transform_dataframe)
        .and_then(df_to_md)?;

    let _ = update_readme(adapter, readme_table, "./README.md");

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
