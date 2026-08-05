use crate::core::adapters::IoValue;
use chrono::{Duration, Utc};
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

pub(crate) fn yesterday() -> String {
    (Utc::now().date_naive() - Duration::days(1))
        .format("%Y-%m-%d")
        .to_string()
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

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use crate::core::{
        adapters::IoValue,
        domain::transform::{
            package_stats_to_readme_table, parse_package_stats, yesterday, PackageStats,
            ReadMeTable,
        },
    };
    use serde_json::json;
    use test_case::test_case;

    fn given_a_vec_of_io_values_when_called_with_yesterday_then_return_vec_of_package_stats(
    ) -> (Vec<IoValue>, String, Vec<PackageStats>) {
        let yesterday = yesterday();
        let values = vec![
            IoValue::Json(json!({
                "id": "some-package",
                "total_downloads": 100,
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
            IoValue::Json(json!({
                "id": "some-other-package",
                "total_downloads": 200,
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
        ];

        let expected_result = vec![
            PackageStats::new("some-package".to_string(), 100, 60),
            PackageStats::new("some-other-package".to_string(), 200, 20),
        ];

        (values, yesterday, expected_result)
    }

    #[test_case(
        given_a_vec_of_io_values_when_called_with_yesterday_then_return_vec_of_package_stats()
    )]
    fn test_parse_package_stats(args: (Vec<IoValue>, String, Vec<PackageStats>)) {
        let (values, yesterday, expected_result) = args;
        let res = parse_package_stats(&values, &yesterday);
        assert_eq!(res, expected_result);
    }

    fn given_a_vec_of_package_stats_when_called_then_return_valid_readme_table(
    ) -> (Vec<PackageStats>, ReadMeTable) {
        let package_stats = vec![
            PackageStats::new("a".to_string(), 100, 50),
            PackageStats::new("b".to_string(), 200, 10),
        ];
        let expected_res = ReadMeTable::new(
            vec![
                "total downloads: `300`\n",
                "yesterday downloads: `60`\n",
                "### breakdown by package",
                "| package | total_downloads | yesterday_downloads |",
                "| --- | --- | --- |",
                "| b | 200 | 10 |",
                "| a | 100 | 50 |",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        );
        (package_stats, expected_res)
    }

    #[test_case(given_a_vec_of_package_stats_when_called_then_return_valid_readme_table())]
    fn test_package_stats_to_readme_table(args: (Vec<PackageStats>, ReadMeTable)) {
        let (input_data, expected_result) = args;
        let res = package_stats_to_readme_table(input_data);
        assert_eq!(res, expected_result);
    }
}
