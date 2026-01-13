# pepy-tech-stats

## python packages
total downloads: `40757`

yesterday downloads: `79`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| readme-update | 10631 | 10 |
| repo-mapper-rs | 10315 | 2 |
| class-inspector | 8653 | 13 |
| danom | 4813 | 34 |
| headline | 3045 | 4 |
| spaghettree | 1680 | 2 |
| repo-mapper | 996 | 1 |
| io-adapters | 624 | 13 |
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
