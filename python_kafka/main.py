import base.python as base
from kafka import (
    KafkaConsumer,
    KafkaProducer
)
import time

LOG = base.log.init("test_kafka")
TOPIC_NAME = "test"
KAFKA_SERVER = "localhost:9092"

consumer = KafkaConsumer(TOPIC_NAME)
producer = KafkaProducer(bootstrap_servers=KAFKA_SERVER)

def main():
    while True:
        producer.send(TOPIC_NAME, b"test message2")
        producer.send(TOPIC_NAME, b"test message2")
        producer.send(TOPIC_NAME, b"test message2")
        producer.send(TOPIC_NAME, b"test message2")
        producer.send(TOPIC_NAME, b"test message2")
        LOG.info("sent message")
        LOG.info("reading messages")
        for message in consumer:
            LOG.info(message)
        time.sleep(1)

main()