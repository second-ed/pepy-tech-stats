# pepy-tech-stats

## python packages
total downloads: `40530`

yesterday downloads: `90`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| readme-update | 10582 | 0 |
| repo-mapper-rs | 10275 | 2 |
| class-inspector | 8630 | 10 |
| danom | 4735 | 66 |
| headline | 3041 | 0 |
| spaghettree | 1678 | 0 |
| repo-mapper | 993 | 0 |
| io-adapters | 596 | 12 |
::


# Repo map
```
├── .github
│   └── workflows
│       ├── ci_tests.yaml
│       └── update_table.yaml
├── .pytest_cache
│   └── README.md
├── python
│   └── pepy_tech_stats
│       ├── core
│       │   ├── __init__.py
│       │   ├── constants.py
│       │   └── logger.py
│       ├── __init__.py
│       └── __main__.py
├── src
│   ├── core
│   │   ├── adapters
│   │   │   ├── io_adapter_builder.rs
│   │   │   ├── io_adapters.rs
│   │   │   ├── io_funcs.rs
│   │   │   ├── io_params.rs
│   │   │   └── mod.rs
│   │   ├── domain
│   │   │   ├── errors.rs
│   │   │   ├── extract_project_stats.rs
│   │   │   ├── mod.rs
│   │   │   ├── transform.rs
│   │   │   └── update_readme.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
├── tests
│   └── integration_tests.rs
├── .pre-commit-config.yaml
├── Cargo.lock
├── Cargo.toml
├── README.md
├── pyproject.toml
├── ruff.toml
└── uv.lock
::
```
