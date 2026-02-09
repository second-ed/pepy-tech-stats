# pepy-tech-stats

## python packages
total downloads: `46687`

yesterday downloads: `499`

### breakdown by package
| package         | total_downloads | yesterday_downloads |
|-----------------|-----------------|---------------------|
| repo-mapper-rs  | 12559           | 120                 |
| readme-update   | 11204           | 61                  |
| class-inspector | 8927            | 15                  |
| danom           | 6431            | 244                 |
| headline        | 3096            | 2                   |
| spaghettree     | 1800            | 3                   |
| io-adapters     | 1202            | 31                  |
| repo-mapper     | 1047            | 5                   |
| papertrail      | 421             | 18                  |
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
