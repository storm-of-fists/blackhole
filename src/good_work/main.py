"""
TODO: generate typescript bindings (or js) https://fastapi.tiangolo.com/advanced/generate-clients/#generate-a-typescript-client-with-custom-operation-ids
TODO: impl auth

Figma: https://www.figma.com/design/kAac1fjfE2cHELzKygSZux/Good-Work?node-id=0-1&t=4QHaMIQKjYGHmqfi-1
"""

from contextlib import asynccontextmanager
from fastapi import FastAPI, HTTPException, status
from pydantic import BaseModel
import asyncpg
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


DATABASE_URL = "postgresql://clatham:admin@localhost:5432/good_work"


@asynccontextmanager
async def lifespan(app: FastAPI):
    # Load the ML model
    startup()
    yield
    # Clean up the ML models and release the resources
    shutdown()


def startup():
    return


def shutdown():
    return


class GoodWorkModel(BaseModel):
    """
    Base class for all Good Work requests.

    Empty right now, but this is used for top level design reference.

    - Dont put timestamps in client payloads. Use the server for timestamps.
    """


class LoggedInRequest(GoodWorkModel):
    user: str
    token: str


class CreateWork(LoggedInRequest):
    """
    Creates a Work. All it needs is a requester and a name for the work.

    Additional settings can prevent duplicate Work names.
    """

    # The name of the Work.
    name: str


class AddLog(LoggedInRequest):
    detail: str


class Assignee(GoodWorkModel):
    user: str


class Approver(GoodWorkModel):
    user: str


class AddPhase(LoggedInRequest):
    name: str
    assignees: list[Assignee]
    approvers: list[Approver]
    # requirements: list[str]


app = FastAPI(lifespan=lifespan, redoc_url=None)


@app.get("/")
def root():
    return {"Hello": "World"}


@app.post("/", status_code=status.HTTP_201_CREATED)
async def create_work(create_work: CreateWork):
    logger.info(create_work)


@app.get("/diagnostic/api", status_code=status.HTTP_200_OK)
async def diagnostic_api():
    """
    Returns healthcheck and other runtime info about the API.
    """
    return


@app.get("/diagnostic/database", status_code=status.HTTP_200_OK)
async def diagnostic_database():
    """
    Returns healthcheck and other runtime info about the database.
    """
    try:
        # Establish a connection
        conn = await asyncpg.connect(DATABASE_URL)  # type: ignore

        # Execute a simple query (e.g., SELECT 1) to verify connection
        # This is a lightweight way to check if the database is reachable and responsive
        await conn.execute("SELECT 1")

        # Close the connection
        await conn.close()

        return {"status": "ok", "database": "connected"}
    except Exception as e:
        # If any error occurs during connection or query, the database is considered unhealthy
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail=f"Database connection failed: {e}",
        )
