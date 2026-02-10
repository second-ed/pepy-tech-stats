# pepy-tech-stats

## python packages
total downloads: `46848`

yesterday downloads: `161`

### breakdown by package
| package         | total_downloads | yesterday_downloads |
|-----------------|-----------------|---------------------|
| repo-mapper-rs  | 12564           | 5                   |
| readme-update   | 11204           | 0                   |
| class-inspector | 8938            | 11                  |
| danom           | 6545            | 114                 |
| headline        | 3097            | 1                   |
| spaghettree     | 1802            | 2                   |
| io-adapters     | 1215            | 13                  |
| repo-mapper     | 1050            | 3                   |
| papertrail      | 433             | 12                  |
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
