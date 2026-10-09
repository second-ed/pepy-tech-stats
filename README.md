# pepy-tech-stats

## python packages
total downloads: `95305`

yesterday downloads: `96`

yesterday date: `2026-10-08`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| repo-mapper-rs | 25102 | 30 |
| danom | 23964 | 11 |
| readme-update | 16903 | 10 |
| class-inspector | 11014 | 27 |
| io-adapters | 7239 | 3 |
| headline | 3790 | 0 |
| spaghettree | 3078 | 9 |
| papertrail | 2482 | 6 |
| repo-mapper | 1733 | 0 |
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
│   │   │   ├── mod.rs
│   │   │   └── request_adapter.rs
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

(generated with repo-mapper-rs)
::
```
