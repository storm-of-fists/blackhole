import base.python.base as base
import asyncio
from kafka import (
    KafkaConsumer,
    KafkaProducer
)

# LOG = base.log.init("kafka")
TOPIC_NAME = "test"
KAFKA_SERVER = "localhost:9092"

consumer = KafkaConsumer(TOPIC_NAME)
producer = KafkaProducer(bootstrap_servers=KAFKA_SERVER)

async def main():
    # LOG.info("starting kafka loop")
    while True:
        producer.send(TOPIC_NAME, b"test message")
        print("sent message")
        # producer.flush()
        # await asyncio.sleep(1.0)
        print("reading messages")
        for message in consumer:
            print(message)
            # LOG.info(message.value)
        # asyncio.sleep(1.0)
    

asyncio.run(main())
