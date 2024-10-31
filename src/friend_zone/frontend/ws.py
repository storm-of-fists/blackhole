import asyncio
import json

import websockets



async def handler(websocket):
    NICE = 0
    
    while True:
        cool = {
            "id": NICE,
            12: "12",
            13: "13"
        }
        message = await websocket.recv()
        # print(message.decode("utf-8"))
        print(message)
        await websocket.send(json.dumps(cool))
        NICE += 1


async def main():
    async with websockets.serve(handler, "", 8001):
        await asyncio.Future()  # run forever


if __name__ == "__main__":
    asyncio.run(main())