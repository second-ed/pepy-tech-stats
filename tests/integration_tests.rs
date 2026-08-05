use chrono::{Duration, Utc};
use pepy_tech_stats::core::{
    adapters::{
        get_fake_adapter, Adapter, FakeFileMap, FakeRequestAdapter, FakeResponseMap, FileType,
        IoValue,
    },
    run,
};
use serde_json::json;
use std::path::PathBuf;
use test_case::test_case;

fn get_api_response(
    project: &str,
    total_downloads: u32,
    yesterday_v1: u32,
    yesterday_v2: u32,
) -> IoValue {
    let yesterday = (Utc::now().date_naive() - Duration::days(1)).to_string();
    IoValue::Json(json!({
        "id": project,
        "total_downloads": total_downloads,
        "versions": ["0.1.0", "0.2.0"],
        "downloads": {
            yesterday.clone(): {
                "0.1.0": yesterday_v1,
                "0.2.0": yesterday_v2
            },
            "2026-01-01": {
                "0.1.0": 5,
                "0.2.0": 20
            },
        },
    }))
}

fn case_1_files() -> (FakeFileMap, FakeResponseMap) {
    let fake_files = vec![
        (
            "./README.md",
            IoValue::Str(
                "## python packages\ntotal downloads: `15`\n\nyesterday downloads: `15`\n\n### breakdown by package\n| package | total_downloads | yesterday_downloads |\n| --- | --- | --- |\n| a | 10 | 10 |\n| b | 5 | 5 |\n\n::".to_string()
            ),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (PathBuf::from(k), v))
    .collect::<FakeFileMap>();

    let fake_responses = vec![
        (
            "https://api.pepy.tech/api/v2/projects/a",
            get_api_response("a", 200, 10, 5),
        ),
        (
            "https://api.pepy.tech/api/v2/projects/b",
            get_api_response("a", 300, 0, 0),
        ),
        (
            "https://api.pepy.tech/api/v2/projects/c",
            get_api_response("c", 100, 50, 0),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect::<FakeResponseMap>();

    (fake_files, fake_responses)
}

struct TestCase {
    projects: Vec<String>,
    expected_readme: IoValue,
}

impl TestCase {
    pub fn new(projects: Vec<&str>, expected_readme: &str) -> Self {
        Self {
            projects: projects.into_iter().map(str::to_string).collect(),
            expected_readme: IoValue::Str(expected_readme.to_string()),
        }
    }
}

fn case_1() -> TestCase {
    TestCase::new(vec!["a"], "## python packages\ntotal downloads: `200`\n\nyesterday downloads: `15`\n\n### breakdown by package\n| package | total_downloads | yesterday_downloads |\n| --- | --- | --- |\n| a | 200 | 15 |\n::")
}

#[tokio::test]
#[test_case(case_1_files(), &case_1())]
async fn test_run(input_data: (FakeFileMap, FakeResponseMap), case: &TestCase) {
    let (files, responses) = input_data;
    let mut adapter = get_fake_adapter(files);
    let client = FakeRequestAdapter::new(responses);

    let _res = run(&mut adapter, &client, &case.projects, "abc-123".to_string()).await;
    let readme = adapter.read(&PathBuf::from("./README.md"), FileType::Str);

    assert!(readme.is_ok());
    assert_eq!(readme.unwrap(), case.expected_readme);
}
