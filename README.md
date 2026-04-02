# pepy-tech-stats

## python packages
total downloads: `56023`

yesterday downloads: `109`

### breakdown by package
| package         | total_downloads | yesterday_downloads |
|-----------------|-----------------|---------------------|
| repo-mapper-rs  | 13905           | 7                   |
| readme-update   | 12168           | 1                   |
| danom           | 10376           | 19                  |
| class-inspector | 9365            | 40                  |
| headline        | 3234            | 2                   |
| io-adapters     | 2563            | 21                  |
| spaghettree     | 2056            | 1                   |
| papertrail      | 1203            | 17                  |
| repo-mapper     | 1153            | 1                   |
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

(generated with repo-mapper-rs)
::
```
