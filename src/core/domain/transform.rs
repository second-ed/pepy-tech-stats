use crate::core::adapters::IoValue;
use serde_json::Value;

pub(crate) fn parse_package_stats(values: &[IoValue], yesterday: &String) -> Vec<PackageStats> {
    values
        .iter()
        .map(|value| match value {
            IoValue::Json(j) => j,
            IoValue::Str(_) => unreachable!(),
        })
        .map(|value| PackageStats::from_request(value, yesterday))
        .collect::<Vec<PackageStats>>()
}

pub(crate) fn package_stats_to_readme_table(mut package_stats: Vec<PackageStats>) -> ReadMeTable {
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
pub(crate) struct PackageStats {
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
pub(crate) struct ReadMeTable {
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
