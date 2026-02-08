from __future__ import annotations

import argparse
import asyncio
import datetime as dt
import itertools
import re
import time
from itertools import batched
from pathlib import Path
from typing import Any

import attrs
import httpx
import polars as pl
from danom import Result, Stream, safe

from pepy_tech_stats.core.constants import (
    BASE,
    PROJECT_STATS_ENDPOINT,
    REQUESTS_PER_MIN,
)
from pepy_tech_stats.core.logger import logger


async def main(client: AsyncPepyStatsClient, projects: list[str]) -> None:
    res = (
        Result.unit(await process_project_stats(client, projects))
        .and_then(responses_to_jsons)
        .and_then(create_readme_table)
        .and_then(update_readme)
    )

    if not res.is_ok():
        raise res.error


@attrs.define
class AsyncPepyStatsClient:
    api_key: str = attrs.field(repr=False)
    base: str = attrs.field(default=BASE)
    project_endpoint: str = attrs.field(default=PROJECT_STATS_ENDPOINT)
    client: httpx.AsyncClient = attrs.field(init=False)

    def __attrs_post_init__(self) -> None:
        self.client = httpx.AsyncClient(headers={"X-API-Key": self.api_key})

    async def get_batch_project_stats(self, projects: list[str]) -> list[httpx.Response]:
        requests = [
            self.client.get(url=f"{self.base}{self.project_endpoint.format(project=project)}") for project in projects
        ]
        return await asyncio.gather(*requests)


async def process_project_stats(
    client: AsyncPepyStatsClient, projects: list[str], requests_per_min: int = REQUESTS_PER_MIN
) -> itertools.chain:
    batches, results = batched(projects, requests_per_min), []

    for idx, batch in enumerate(batches):
        if idx > 0:
            # only want to sleep after we've exceeded the max requests for the first time
            # use blocking time.sleep to avoid rate limit
            time.sleep(60)  # noqa: ASYNC251
        results.append(await client.get_batch_project_stats(batch))

    return itertools.chain.from_iterable(results)


@safe
def responses_to_jsons(responses: list[httpx.Response]) -> tuple[dict[str, Any]]:
    oks, errs = Stream.from_iterable(responses).partition(lambda x: x.is_success)
    failed_gets = errs.tap(lambda x: logger.error(vars(x))).collect()

    if failed_gets:
        raise RuntimeError(f"Failed to get stats for {failed_gets = }")
    return oks.tap(lambda x: logger.info(f"{x.status_code} | {x.request}")).map(lambda x: x.json()).collect()


@safe
def create_readme_table(project_stats: tuple[dict[str, str], ...]) -> str:
    yesterday = str(dt.datetime.now(tz=dt.UTC).date() - dt.timedelta(1))

    df = (
        pl.DataFrame(project_stats)
        .lazy()
        .rename({"id": "package"})
        .unnest("downloads")
        .with_columns([pl.sum_horizontal(pl.col(yesterday).struct.field("*")).alias("yesterday_downloads")])
        .select("package", "total_downloads", "yesterday_downloads")
        .sort("total_downloads", descending=True)
        .collect()
    )

    with pl.Config(
        tbl_formatting="MARKDOWN",
        tbl_hide_column_data_types=True,
        tbl_hide_dataframe_shape=True,
    ):
        return "\n".join(
            [
                f"total downloads: `{df['total_downloads'].sum()}`\n",
                f"yesterday downloads: `{df['yesterday_downloads'].sum()}`\n",
                "### breakdown by package",
                repr(df),
            ]
        )


@safe
def update_readme(readme_table: str, readme_path: str = "./README.md") -> None:
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
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("--api-key", type=str)
    args = parser.parse_args()

    client = AsyncPepyStatsClient(args.api_key)
    projects = [
        "class-inspector",
        "danom",
        "headline",
        "io-adapters",
        "papertrail",
        "readme-update",
        "repo-mapper",
        "repo-mapper-rs",
        "spaghettree",
    ]
    asyncio.run(main(client, projects))
