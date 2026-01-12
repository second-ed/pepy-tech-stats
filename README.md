# pepy-tech-stats

## python packages
total downloads: `40678`

yesterday downloads: `148`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| readme-update | 10621 | 39 |
| repo-mapper-rs | 10313 | 38 |
| class-inspector | 8640 | 10 |
| danom | 4779 | 44 |
| headline | 3041 | 0 |
| spaghettree | 1678 | 0 |
| repo-mapper | 995 | 2 |
| io-adapters | 611 | 15 |
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
