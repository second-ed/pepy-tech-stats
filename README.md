# pepy-tech-stats

## python packages
total downloads: `41439`

yesterday downloads: `107`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| readme-update | 10783 | 22 |
| repo-mapper-rs | 10416 | 1 |
| class-inspector | 8705 | 22 |
| danom | 5083 | 51 |
| headline | 3054 | 0 |
| spaghettree | 1709 | 0 |
| repo-mapper | 1008 | 0 |
| io-adapters | 681 | 11 |
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
