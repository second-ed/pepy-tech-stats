# pepy-tech-stats

## python packages
total downloads: `95838`

yesterday downloads: `26`

yesterday date: `2026-10-11`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| repo-mapper-rs | 25133 | 0 |
| danom | 24312 | 24 |
| readme-update | 16967 | 0 |
| class-inspector | 11039 | 2 |
| io-adapters | 7271 | 0 |
| headline | 3791 | 0 |
| spaghettree | 3094 | 0 |
| papertrail | 2492 | 0 |
| repo-mapper | 1739 | 0 |
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
