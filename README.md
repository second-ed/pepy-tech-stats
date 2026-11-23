# pepy-tech-stats

## python packages
total downloads: `103963`

yesterday downloads: `328`

yesterday date: `2026-11-22`

### breakdown by package
| package | total_downloads | yesterday_downloads |
| --- | --- | --- |
| repo-mapper-rs | 29045 | 133 |
| danom | 26293 | 5 |
| readme-update | 17856 | 75 |
| class-inspector | 11312 | 35 |
| io-adapters | 7714 | 40 |
| headline | 3900 | 6 |
| spaghettree | 3302 | 18 |
| papertrail | 2719 | 9 |
| repo-mapper | 1822 | 7 |
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
