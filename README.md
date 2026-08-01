# pepy-tech-stats

## python packages
total downloads: `84194`

yesterday downloads: `317`

### breakdown by package
| package         | total_downloads | yesterday_downloads |
|-----------------|-----------------|---------------------|
| repo-mapper-rs  | 21585           | 215                 |
| danom           | 20284           | 39                  |
| readme-update   | 15334           | 0                   |
| class-inspector | 10451           | 6                   |
| io-adapters     | 6342            | 47                  |
| headline        | 3630            | 1                   |
| spaghettree     | 2820            | 9                   |
| papertrail      | 2131            | 0                   |
| repo-mapper     | 1617            | 0                   |
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
