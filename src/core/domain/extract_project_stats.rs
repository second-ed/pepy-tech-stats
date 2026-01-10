use crate::core::adapters::{Adapter, FileType, IoValue};
use crate::core::domain::errors::PepyStatsError;
use itertools::Itertools;
use log;
use std::path::PathBuf;
use std::{thread, time::Duration as SleepDuration};

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
) -> Result<Vec<IoValue>, PepyStatsError> {
    let mut results: Vec<Result<Vec<IoValue>, PepyStatsError>> = Vec::new();

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
        .collect::<Result<Vec<Vec<IoValue>>, PepyStatsError>>()
        .map(|batches| batches.into_iter().flatten().collect())
}

#[inline(always)]
fn process_batch_project_stats(
    adapter: &mut impl Adapter,
    projects: Vec<&String>,
) -> Result<Vec<IoValue>, PepyStatsError> {
    let results: Result<Vec<IoValue>, PepyStatsError> = projects
        .iter()
        .map(|project| PepyUrl::new(project).into_url())
        .map(PathBuf::from)
        .map(|url| adapter.read(&url, FileType::ApiCall))
        .collect();
    results
}
