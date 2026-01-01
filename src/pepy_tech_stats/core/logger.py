import logging
import os
from logging.handlers import RotatingFileHandler

from pepy_tech_stats.core.constants import REPO_ROOT

logger = logging.getLogger(__name__)
logger.propagate = False


formatter = logging.Formatter(
    "%(asctime)s | %(levelname)-8s [%(filename)s:%(lineno)d:%(funcName)s] %(message)s"
)
logger.setLevel(logging.DEBUG)
stream_handler = logging.StreamHandler()
stream_handler.setLevel(logging.INFO)
stream_handler.setFormatter(formatter)
logger.addHandler(stream_handler)

if os.getenv("LOGGING_ENABLED", "false").lower() == "true":
    log_path = f"{REPO_ROOT}/logs/app.log"
    os.makedirs(os.path.dirname(log_path), exist_ok=True)

    file_handler = RotatingFileHandler(log_path, maxBytes=2_000_000, backupCount=1)
    file_handler.setLevel(logging.DEBUG)
    file_handler.setFormatter(formatter)
    logger.addHandler(file_handler)
