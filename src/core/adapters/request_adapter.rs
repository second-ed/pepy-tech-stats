use std::collections::HashMap;

use crate::core::{adapters::IoValue, domain::errors::PepyStatsError};

pub trait ApiRequester {
    #[allow(async_fn_in_trait)]
    async fn get(&self, url: &str, api_key: &str) -> Result<IoValue, PepyStatsError>;
}

pub struct ReqwestAdapter {
    client: reqwest::Client,
}

impl ReqwestAdapter {
    #[must_use]
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl ApiRequester for ReqwestAdapter {
    async fn get(&self, url: &str, api_key: &str) -> Result<IoValue, PepyStatsError> {
        let value = self
            .client
            .get(url)
            .header("X-API-Key", api_key)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        Ok(IoValue::Json(value))
    }
}

pub type FakeResponseMap = HashMap<String, IoValue>;

pub struct FakeRequestAdapter {
    responses: FakeResponseMap,
}

impl FakeRequestAdapter {
    #[must_use]
    pub fn new(responses: FakeResponseMap) -> Self {
        Self { responses }
    }
}

impl ApiRequester for FakeRequestAdapter {
    async fn get(&self, url: &str, _api_key: &str) -> Result<IoValue, PepyStatsError> {
        self.responses
            .get(url)
            .cloned()
            .ok_or_else(|| PepyStatsError::NotFound(url.into()))
    }
}
