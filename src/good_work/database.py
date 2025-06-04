import asyncpg
from logger import logger
from fastapi import HTTPException, status
from typing import AsyncGenerator

_pool: asyncpg.Pool | None = None
DATABASE_URL = "postgresql://clatham:admin@localhost:5432/good_work"


async def initialize_database_pool():
    global _pool

    if not _pool:
        try:
            _pool = await asyncpg.create_pool(
                dsn=DATABASE_URL, min_size=1, max_size=8, timeout=30
            )
            logger.debug("PostgeSQL connection pool created.")
        except Exception as e:
            print(f"Failed to connect to PostgreSQL: {e}")
            raise


async def close_database_pool():
    global _pool

    if _pool:
        await _pool.close()
        _pool = None
        logger.debug("PostgreSQL connection pool closed.")


async def get_database_connection() -> AsyncGenerator[asyncpg.Connection | None, None]:
    global _pool

    if not _pool:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="Database pool not initialized.",
        )

    connection: asyncpg.Connection | None = None

    try:
        connection = await _pool.acquire()
        yield connection
    # TODO does this only catch this error type? what about others?
    except asyncpg.exceptions.PostgresError as e:
        # Log more detail server side, protect your data.
        logger.error(f"Database error: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            # Keep this detail light.
            detail="Database error occurred.",
        )
    finally:
        if connection:
            await _pool.release(connection)
