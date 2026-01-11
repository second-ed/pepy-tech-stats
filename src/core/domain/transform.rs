use crate::core::{adapters::IoValue, domain::errors::PepyStatsError};
use chrono::{Duration, Utc};
use log;
use polars::{
    lazy::dsl::sum_horizontal,
    prelude::{col, DataFrame, SortMultipleOptions, *},
};

#[inline(always)]
pub fn responses_to_df(values: Vec<IoValue>) -> Result<DataFrame, PepyStatsError> {
    let json_rows: Vec<serde_json::Value> = values
        .into_iter()
        .map(|v| match v {
            IoValue::Json(j) => j,
            _ => unreachable!(),
        })
        .collect();
    Ok(JsonReader::new(std::io::Cursor::new(serde_json::to_vec(&json_rows)?)).finish()?)
}

#[inline(always)]
pub fn transform_dataframe(df: DataFrame) -> Result<DataFrame, PepyStatsError> {
    let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();
    log::info!("yesterday: {:?}", yesterday);

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

#[derive(Debug)]
pub struct ReadMeTable {
    lines: Vec<String>,
}

impl ReadMeTable {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    pub fn into_string(self) -> String {
        self.lines.join("\n")
    }
}

pub fn df_to_md(df: DataFrame) -> Result<ReadMeTable, PepyStatsError> {
    let packages = df.column("package")?.str()?;
    let totals = df.column("total_downloads")?.i64()?;
    let yesterday = df.column("yesterday_downloads")?.i64()?;

    let mut lines = vec![
        format!(
            "total downloads: `{}`\n",
            df.column("total_downloads")?.i64()?.sum().unwrap_or(0),
        ),
        format!(
            "yesterday downloads: `{}`\n",
            df.column("yesterday_downloads")?.i64()?.sum().unwrap_or(0),
        ),
        "### breakdown by package".to_string(),
        "| package | total_downloads | yesterday_downloads |".to_string(),
        "| --- | --- | --- |".to_string(),
    ];

    for i in 0..df.height() {
        lines.push(format!(
            "| {} | {} | {} |",
            packages.get(i).unwrap_or(""),
            totals.get(i).unwrap_or(0),
            yesterday.get(i).unwrap_or(0)
        ));
    }

    Ok(ReadMeTable::new(lines))
}

#[cfg(test)]
mod tests {
    use crate::core::{
        adapters::IoValue,
        domain::{
            errors::PepyStatsError,
            transform::{df_to_md, responses_to_df, transform_dataframe, ReadMeTable},
        },
    };
    use chrono::{Duration, Utc};
    use polars::prelude::{col, *};
    use serde_json::json;
    use test_case::test_case;

    fn mock_responses() -> Result<Vec<IoValue>, PepyStatsError> {
        let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();

        Ok(vec![
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
        ])
    }

    fn mock_responses_md_table() -> ReadMeTable {
        ReadMeTable::new(
            vec![
                "total downloads: `300`\n",
                "yesterday downloads: `80`\n",
                "### breakdown by package",
                "| package | total_downloads | yesterday_downloads |",
                "| --- | --- | --- |",
                "| some-other-package | 200 | 20 |",
                "| some-package | 100 | 60 |",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        )
    }

    #[test_case(mock_responses(), mock_responses_md_table())]
    fn test_responses_to_transformed_df(
        input_data: Result<Vec<IoValue>, PepyStatsError>,
        expected_result: ReadMeTable,
    ) {
        let res = input_data
            .and_then(responses_to_df)
            .and_then(transform_dataframe)
            .and_then(df_to_md);

        assert!(&res.is_ok());
        assert_eq!(res.unwrap().lines, expected_result.lines);
    }
}
