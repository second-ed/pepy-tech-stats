use crate::core::adapters::{Adapter, FileType, IoError, IoValue};
use chrono::{Duration, Utc};
use itertools::Itertools;
use log;
use polars::lazy::dsl::sum_horizontal;
use polars::prelude::*;
use polars::prelude::{col, DataFrame, SortMultipleOptions};
use std::path::PathBuf;
use std::{thread, time::Duration as SleepDuration};

pub enum RetCode {
    OK,
    ERR,
}

pub const BASE_URL: &str = "https://api.pepy.tech";
pub const PROJECT_STATS_ENDPOINT: &str = "/api/v2/projects/";
pub const REQUESTS_PER_MIN: usize = 10;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PepyUrl {
    base_url: String,
    project: String,
}

impl PepyUrl {
    pub fn new(project: &str) -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            project: project.to_string(),
        }
    }

    pub fn into_url(self) -> String {
        let project_endpoint = format!("{}{}", PROJECT_STATS_ENDPOINT, self.project);
        format!(
            "{base_url}{project_endpoint}",
            base_url = self.base_url,
            project_endpoint = project_endpoint
        )
    }
}

pub fn process_project_stats(
    adapter: &mut impl Adapter,
    projects: Vec<String>,
    requests_per_min: usize,
) -> Result<Vec<IoValue>, IoError> {
    let mut results: Vec<Result<Vec<IoValue>, IoError>> = Vec::new();

    for (idx, batch) in projects
        .iter()
        .chunks(requests_per_min)
        .into_iter()
        .enumerate()
    {
        if idx > 0 {
            // only sleep after we've exceeded the max requests once
            thread::sleep(SleepDuration::from_secs(60));
        }

        let batch: Vec<_> = batch.collect();
        results.push(process_batch_project_stats(adapter, batch));
    }
    results
        .into_iter()
        .collect::<Result<Vec<Vec<IoValue>>, IoError>>()
        .map(|batches| batches.into_iter().flatten().collect())
}

#[inline(always)]
fn process_batch_project_stats(
    adapter: &mut impl Adapter,
    projects: Vec<&String>,
) -> Result<Vec<IoValue>, IoError> {
    let results: Result<Vec<IoValue>, IoError> = projects
        .iter()
        .map(|project| PepyUrl::new(project).into_url())
        .map(PathBuf::from)
        .map(|url| adapter.read(&url, FileType::Json))
        .collect();
    results
}

pub fn responses_to_df(values: Vec<IoValue>) -> Result<DataFrame, IoError> {
    let json_rows: Vec<serde_json::Value> = values
        .into_iter()
        .map(|v| match v {
            IoValue::Json(j) => j,
            _ => unreachable!(),
        })
        .collect();
    Ok(JsonReader::new(std::io::Cursor::new(serde_json::to_vec(&json_rows)?)).finish()?)
}

pub fn transform_dataframe(df: DataFrame) -> Result<DataFrame, IoError> {
    let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();
    log::info!("yesterday: {:?}", yesterday);
    dbg!(&df);

    let lf = df
        .lazy()
        .rename(vec!["id".to_string()], vec!["package".to_string()], true)
        .unnest(Selector::ByName {
            names: ["downloads".into()].into(),
            strict: true,
        })
        .with_column(
            sum_horizontal(vec![col(yesterday).struct_().field_by_name("*")], true)?
                .alias("yesterday_downloads"),
        )
        .select([
            col("package"),
            col("total_downloads"),
            col("yesterday_downloads"),
        ])
        .sort(
            ["total_downloads".to_string()],
            SortMultipleOptions::default()
                .with_maintain_order(true)
                .with_multithreaded(true)
                .with_order_descending(true),
        );

    // collect lazy frame into DataFrame
    let df: DataFrame = lf.collect()?;

    Ok(df)
}
