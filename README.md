# pepy-tech-stats

## python packages
total downloads: `41600`

yesterday downloads: `161`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| readme-update | 10799 | 16 |
| repo-mapper-rs | 10432 | 16 |
| class-inspector | 8762 | 57 |
| danom | 5117 | 34 |
| headline | 3056 | 2 |
| spaghettree | 1723 | 14 |
| repo-mapper | 1010 | 2 |
| io-adapters | 701 | 20 |
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
