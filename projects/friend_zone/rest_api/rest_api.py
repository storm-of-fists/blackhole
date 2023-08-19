from fastapi import (FastAPI, WebSocket)
from contextlib import asynccontextmanager
import uvicorn
import asyncio
import random
import time
from redis import asyncio as aioredis
import json


redis_client = aioredis.Redis()

@asynccontextmanager
async def lifespan(app: FastAPI):
    await redis_client
    yield
    await redis_client.close()

app = FastAPI(docs_url="/api/docs", lifespan=lifespan)

@app.get("/api/items/{item_id}")
def read_item(item_id: int, q: str = None):
    return {"item_id": item_id, "q": q}

@app.get("/api/redis/pub")
async def redis_access():
    while True:
        data_dict = {
            "sub1": random.randint(0, 9), 
            "sub2": random.random(), 
            "sub3": "s" * random.randint(0,3), 
            "sub4": 4
        }

        for channel, data in data_dict.items():
            await redis_client.publish(channel, data)

        await asyncio.sleep(0.1)

@app.get("/api/redis/sub")
async def redis_access():
    pub = redis_client.pubsub()

    await pub.subscribe("sub1", "sub2", "sub3", "sub4")
    while True:
        async for message in pub.listen():
            try:
                print(json.loads(message.get("data")))
            except:
                pass


@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    message_wait_time = 0.001
    loop_time = 0.1
    incoming_messages = []
    subscription = {}

    async def _receive():
        try:
            while True:
                message = await websocket.receive_json()
                incoming_messages.append(message)
        except (TimeoutError, asyncio.exceptions.CancelledError):
            pass

    await websocket.accept()

    current_time = time.time()
    while True:
        await asyncio.wait_for(_receive(), timeout=message_wait_time)

        if incoming_messages:
            for message in incoming_messages:
                sub_channel = message.get("subscribe", None)
                unsub_channel = message.get("unsubscribe", None)
                if sub_channel:
                    subscription.update({sub_channel: random.random()})
                if unsub_channel:
                    if subscription.get(unsub_channel):
                        subscription.pop(unsub_channel)

        if subscription:
            subscription.update((k, random.randint(0, 9)) for k, _ in subscription.items())
            await websocket.send_json(subscription)

        sleep_time = loop_time - (time.time() - current_time)
        print(sleep_time)
        await asyncio.sleep(sleep_time)
        current_time = time.time()


if __name__ == "__main__":
    uvicorn.run("rest_api:app", host="localhost", port=8888)