import tools.python.log.log as logging
import tools.python.zeromq.zeromq as zmq

import threading
import time

log, _, _ = logging.init("zmq_test")

REQUEST_RESPONSE_PORT = 5555
# RADIO_DISH_PORT = 5556
PUB_SUB_PORT = 5557


def _run_threads(threads):
    for thread in threads:
        thread.start()

    for thread in threads:
        thread.join()


def run_response():
    server = zmq.ctx.socket(zmq.REP)
    server.bind(f"tcp://*:{REQUEST_RESPONSE_PORT}")
    message_count = 0

    while message_count < 10:
        message = server.recv()
        log.debug(f"response received: {message}")

        assert message

        time.sleep(0.1)

        server.send_string("World")
        message_count += 1

    server.close()


def run_request():
    client = zmq.ctx.socket(zmq.REQ)
    client.connect(f"tcp://127.0.0.1:{REQUEST_RESPONSE_PORT}")

    for request in range(10):
        log.debug(f"request sent {request} ...")
        client.send_string("Hello")

        message = client.recv()

        assert message

        log.debug(f"request received {request} [ {message} ]")

    client.close()


def request_response_test():
    _run_threads(threading.Thread(target=fun) for fun in (run_response, run_request))


# TODO: draft support needs to be opted into https://github.com/zeromq/pyzmq/blob/ae615d4097ccfbc6b5c17de60355cbe6e00a6065/docs/source/howto/draft.md
# def run_radio():
#     log = lambda msg: log.debug(f"server - {msg}")

#     radio = zmq.ctx.socket(zmq.RADIO)
#     radio.connect(f"udp://*:{RADIO_DISH_PORT}")

#     for _ in 10:
#         log("Radio sent message.")
#         radio.send(b"0", group="test")
#         time.sleep(1)

#     radio.close()


# def run_dish():
#     log = lambda msg: log.debug(f"server - {msg}")

#     dish = zmq.ctx.socket(zmq.DISH)
#     dish.bind(f"udp://127.0.0.1:{RADIO_DISH_PORT}")
#     dish.join("test")

#     message_count_limit = 3
#     message_count = 0

#     while message_count < message_count_limit:
#         log("Dish received message.")
#         dish.recv()
#         message_count += 1

#     dish.close()


# def radio_dish_test():
#     _run_threads(threading.Thread(target=fun) for fun in (run_radio, run_dish))


def run_pub():
    pub = zmq.ctx.socket(zmq.PUB)
    pub.bind(f"tcp://*:{PUB_SUB_PORT}")

    for message_number in range(10):
        log.debug(f"pub sent {message_number}")
        pub.send_string(f"{message_number}")

        time.sleep(0.1)


def run_sub():
    sub = zmq.ctx.socket(zmq.SUB)
    sub.connect(f"tcp://127.0.0.1:{PUB_SUB_PORT}")
    sub.setsockopt(zmq.SUBSCRIBE, b'')
    message_count = 0

    while message_count < 3:
        log.debug("sub waiting for message")
        message = sub.recv()
        log.debug(f"sub received {message}")
        assert message

        time.sleep(0.1)
        message_count += 1


def pub_sub_test():
    _run_threads(threading.Thread(target=fun) for fun in (run_sub, run_pub))


if __name__ == "__main__":
    request_response_test()
    pub_sub_test()
    # radio_dish_test()
