import logging as log
import sys

def init(name, level=log.DEBUG):
    logger = log.getLogger(name)
    logger.setLevel(level)

    print_handler = log.StreamHandler(sys.stdout)
    print_handler.setLevel(level)

    formatter = log.Formatter('[%(name)s %(asctime)s %(levelname)s] - %(message)s')
    print_handler.setFormatter(formatter)

    logger.addHandler(print_handler)

    logger.debug(f"Enabled logging.")

    return logger
