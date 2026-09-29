# pepy-tech-stats

## python packages
total downloads: `93854`

yesterday downloads: `63`

yesterday date: `2026-09-28`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| repo-mapper-rs | 24701 | 5 |
| danom | 23527 | 28 |
| readme-update | 16646 | 2 |
| class-inspector | 10926 | 6 |
| io-adapters | 7117 | 22 |
| headline | 3750 | 0 |
| spaghettree | 3030 | 0 |
| papertrail | 2443 | 0 |
| repo-mapper | 1714 | 0 |
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
