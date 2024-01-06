from logging import *
import sys


def init(name=__file__, level=DEBUG):
    logger = getLogger(name)
    logger.setLevel(level)

    print_handler = StreamHandler(sys.stdout)
    print_handler.setLevel(level)

    formatter = Formatter("[%(name)s %(asctime)s %(levelname)s] - %(message)s")
    print_handler.setFormatter(formatter)

    logger.addHandler(print_handler)

    logger.debug(f"Enabled logging.")

    return logger
