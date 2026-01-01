import itertools
import re
import sys
import time
from collections.abc import Callable
from functools import partial
from itertools import batched
from pathlib import Path

import polars as pl
import requests
from danom import Err, Ok, Stream, safe

from pepy_tech_stats.core.constants import (
    API_KEY,
    BASE,
    PROJECT_STATS_ENDPOINT,
    REPO_ROOT,
    REQUESTS_PER_MIN,
)
from pepy_tech_stats.core.logger import logger


def main(projects: list[str]) -> None:
    res = (
        process_project_stats(
            projects=projects,
            base=BASE,
            project_endpoint=PROJECT_STATS_ENDPOINT,
            requests_per_min=REQUESTS_PER_MIN,
        )
        .and_then(create_readme_table)
        .and_then(update_readme)
    )

    match res:
        case Ok(inner):
            sys.exit(inner)
        case Err(error):
            raise error


@safe
def process_project_stats(
    projects: list[str], base: str, project_endpoint: str, requests_per_min: int
) -> itertools.chain:
    batches = batched(projects, requests_per_min)

    results = []

    for idx, batch in enumerate(batches):
        if idx > 0:
            # only want to sleep after we've exceeded the max requests for the first time
            time.sleep(60)
        results.append(process_batch_project_stats(batch, base, project_endpoint))

    return itertools.chain.from_iterable(results)


def get_project_stats(
    project: str,
    base: str,
    project_endpoint: str,
) -> requests.Response:
    logger.info(f"{locals() = }")
    res = requests.get(
        url=f"{base}{project_endpoint.format(project=project)}",
        headers={"X-API-Key": API_KEY},
        timeout=10,
    )
    logger.info(
        f"{res.request = } | {res.url = } | {res.status_code = } | {res.reason = } | {res.text[:200] = }"
    )
    return res


def process_batch_project_stats(
    projects: list[str],
    base: str,
    project_endpoint: str,
    request_fn: Callable[[str, str, str], requests.Response] = get_project_stats,
) -> tuple[dict[str, str], ...]:
    successes, fails = (
        Stream.from_iterable(projects)
        .map(
            partial(
                request_fn,
                base=base,
                project_endpoint=project_endpoint,
            )
        )
        .partition(lambda x: x.ok)
    )

    failed_gets = fails.tap(lambda x: logger.error(vars(x))).collect()
    if failed_gets:
        raise RuntimeError(f"Failed to get stats for {failed_gets = }")
    return successes.map(lambda x: x.json()).collect()


@safe
def create_readme_table(project_stats: tuple[dict[str, str], ...]) -> str:
    df = (
        pl.DataFrame(project_stats)
        .rename({"id": "package"})
        .select(["package", "total_downloads"])
        .sort("total_downloads", descending=True)
    )

    with pl.Config(
        tbl_formatting="MARKDOWN",
        tbl_hide_column_data_types=True,
        tbl_hide_dataframe_shape=True,
    ):
        return "\n".join(
            [
                f"total downloads: `{df['total_downloads'].sum()}`",
                "### breakdown by package",
                repr(df),
            ]
        )


@safe
def update_readme(readme_table: str, readme_path: str = f"{REPO_ROOT}/README.md") -> None:
    readme_path = Path(readme_path)
    readme_txt = readme_path.read_text(encoding="utf-8")
    pattern = r"(# python packages)(.*?)(::)"
    updated_readme = re.sub(pattern, rf"\1\n{readme_table}\n\3", readme_txt, flags=re.DOTALL)
    
    if readme_txt != updated_readme:
        readme_path.write_text(updated_readme)
        logger.info("updated readme")
        return 1
    
    logger.info("no changes to readme")
    return 0


if __name__ == "__main__":
    projects = [
        "class-inspector",
        "danom",
        "headline",
        "io-adapters",
        "readme-update",
        "repo-mapper",
        "repo-mapper-rs",
        "spaghettree",
    ]
    main(projects)
