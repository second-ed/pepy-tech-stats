use crate::core::{
    adapters::{ApiRequester, IoValue},
    domain::errors::PepyStatsError,
};
use futures::future::try_join_all;
use log;

pub const BASE_URL: &str = "https://api.pepy.tech";
pub const PROJECT_STATS_ENDPOINT: &str = "/api/v2/projects/";
pub const REQUESTS_PER_MIN: usize = 10;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PepyUrl {
    base_url: String,
    project: String,
}

impl PepyUrl {
    #[must_use]
    pub fn new(project: &str) -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            project: project.to_string(),
        }
    }
    #[must_use]
    pub fn into_url(self) -> String {
        let project_endpoint = format!("{}{}", PROJECT_STATS_ENDPOINT, self.project);
        format!(
            "{base_url}{project_endpoint}",
            base_url = self.base_url,
            project_endpoint = project_endpoint
        )
    }
}

pub async fn process_project_stats(
    client: &impl ApiRequester,
    projects: &[String],
    api_key: &str,
    requests_per_min: usize,
) -> Result<Vec<IoValue>, PepyStatsError> {
    if requests_per_min == 0 {
        return Err(PepyStatsError::InvalidRequestsPerMinValue(requests_per_min));
    }

    let mut results: Vec<Vec<IoValue>> = Vec::new();

    let batches: Vec<_> = projects.chunks(requests_per_min).collect();

    for (idx, batch) in batches.iter().enumerate() {
        if idx > 0 {
            log::info!("Sleeping for batch {idx:?}");
            // only sleep after we've exceeded the max requests once
            tokio::time::sleep(std::time::Duration::from_mins(1)).await;
        }

        let requests = batch.iter().map(|project| {
            let url = PepyUrl::new(project).into_url();

            async move {
                log::info!("Sending request to: {url:?}");

                client.get(&url, api_key).await
            }
        });

        let batch_results = try_join_all(requests).await?;
        results.push(batch_results);
    }
    Ok(results.into_iter().flatten().collect())
}
