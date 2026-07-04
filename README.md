# pepy-tech-stats

## python packages
total downloads: `79079`

yesterday downloads: `128`

### breakdown by package
| package         | total_downloads | yesterday_downloads |
|-----------------|-----------------|---------------------|
| repo-mapper-rs  | 19996           | 16                  |
| danom           | 18350           | 61                  |
| readme-update   | 14896           | 7                   |
| class-inspector | 10227           | 3                   |
| io-adapters     | 5856            | 16                  |
| headline        | 3557            | 3                   |
| spaghettree     | 2677            | 3                   |
| papertrail      | 1959            | 14                  |
| repo-mapper     | 1561            | 5                   |
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
