import os
from pathlib import Path

from dotenv import load_dotenv

REPO_ROOT = Path(__file__).absolute().parents[3]

load_dotenv(f"{REPO_ROOT}/envs/.env")

API_KEY = os.getenv("API_KEY")

BASE = "https://api.pepy.tech"
PROJECT_STATS_ENDPOINT = "/api/v2/projects/{project}"
REQUESTS_PER_MIN = 10
