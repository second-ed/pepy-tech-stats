use crate::core::adapters::{IoError, IoValue};
use chrono::{Duration, Utc};
use log;
use polars::lazy::dsl::sum_horizontal;
use polars::prelude::*;
use polars::prelude::{col, DataFrame, SortMultipleOptions};

#[inline(always)]
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

#[inline(always)]
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

#[cfg(test)]
mod tests {
    use crate::core::{
        adapters::{IoError, IoValue},
        domain::transform::{responses_to_df, transform_dataframe},
    };
    use chrono::{Duration, Utc};
    use polars::prelude::col;
    use polars::prelude::*;
    use serde_json::json;
    use test_case::test_case;

    fn mock_responses() -> Result<Vec<IoValue>, IoError> {
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

    #[test_case(mock_responses(), "some_package".to_string(), "total_downloads".to_string(), Some(100))]
    #[test_case(mock_responses(), "some_package".to_string(), "yesterday_downloads".to_string(), Some(20))]
    #[test_case(mock_responses(), "some_other_package".to_string(), "total_downloads".to_string(), Some(200))]
    #[test_case(mock_responses(), "some_other_package".to_string(), "yesterday_downloads".to_string(), Some(60))]
    fn test_responses_to_transformed_df(
        input_data: Result<Vec<IoValue>, IoError>,
        package: String,
        col_name: String,
        expected_result: Option<i64>,
    ) {
        #[inline(always)]
        fn get_i64(df: &DataFrame, package: &str, col_name: &str, row: usize) -> Option<i64> {
            df.clone()
                .lazy()
                .filter(col(PlSmallStr::from("package")).eq(lit(package)))
                .collect()
                .ok()?
                .column(col_name)
                .unwrap()
                .i64()
                .unwrap()
                .get(row)
        }

        let res = input_data
            .and_then(responses_to_df)
            .and_then(transform_dataframe);

        assert!(&res.is_ok());
        assert_eq!(
            get_i64(&res.unwrap(), &package, &col_name, 0),
            expected_result
        );
    }
}
