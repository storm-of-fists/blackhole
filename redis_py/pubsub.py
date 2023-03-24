from redis import Redis
import json
import random
import time


redis_client = Redis()
pub = redis_client.pubsub()
pub.subscribe("sub1", "sub2", "sub3", "sub4")

while True:
    data_dict = {
        "sub1": random.randint(0, 9), 
        "sub2": random.random(), 
        "sub3": "s" * random.randint(0,3), 
        "sub4": 4
    }

    for channel, data in data_dict.items():
        redis_client.publish(channel, data)

    for message in pub.get_message():
        try:
            print(json.loads(message.get("data")))
        except:
            pass
    print("loop")
    time.sleep(1)

redis_client.close()