from logging import *
import sys
import pathlib


def init(name=__file__, level=DEBUG, file=True):
    logger = getLogger(name)
    logger.setLevel(level)
    formatter = Formatter("[%(asctime)s %(levelname)-5s %(name)s] - %(message)s")
    handlers = []

    print_handler = StreamHandler(sys.stdout)
    print_handler.setLevel(level)
    print_handler.setFormatter(formatter)
    handlers.append(print_handler)

    if file:
        file_str_name = file if type(file) == str else name
        file_name = pathlib.Path(f"/tmp/{file_str_name}.log")
        file_handler = FileHandler(file_name)
        file_handler.setLevel(level)
        file_handler.setFormatter(formatter)
        handlers.append(file_handler)

    for handler in handlers:
        logger.addHandler(handler)

    logger.debug(f"Enabled logging.")

    return logger, handlers, formatter
