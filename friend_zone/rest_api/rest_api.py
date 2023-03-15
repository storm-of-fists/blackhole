from fastapi import FastAPI
from fastapi.openapi.docs import get_swagger_ui_html
import uvicorn
import sys
from pathlib import Path
import os


app = FastAPI(docs_url="/api/docs")


@app.get("/api/items/{item_id}")
def read_item(item_id: int, q: str = None):
    return {"item_id": item_id, "q": q}


if __name__ == "__main__":
    uvicorn.run("rest_api:app", host="0.0.0.0", port=8888)