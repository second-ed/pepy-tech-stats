# pepy-tech-stats

## python packages
total downloads: `102243`

yesterday downloads: `203`

yesterday date: `2026-11-14`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| repo-mapper-rs | 28606 | 82 |
| danom | 25725 | 82 |
| readme-update | 17594 | 20 |
| class-inspector | 11205 | 11 |
| io-adapters | 7532 | 3 |
| headline | 3874 | 3 |
| spaghettree | 3241 | 0 |
| papertrail | 2665 | 1 |
| repo-mapper | 1801 | 1 |
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
