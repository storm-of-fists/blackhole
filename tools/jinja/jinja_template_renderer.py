import base.python.log as logging
import argparse
from jinja2 import Environment, StrictUndefined, FileSystemLoader
from pathlib import Path

# class PickledConfig:
# class YamlConfig:
# class DictConfig:

if __name__ == "__main__":
    # SET UP LOGGING
    log = logging.init("jinja_templater")

    # PARSE ARGS
    log.debug("Parsing args.")
    parser = argparse.ArgumentParser(
        prog="Jinja Template Renderer",
        description="Parse a jinja template and render it with bazel.",
    )
    parser.add_argument("template_path")
    parser.add_argument("config")

    args = parser.parse_args()

    template_path = Path(args.template_path)

    # CONFIGURE JINJA
    log.debug("Setting up jinja env.")
    jinja_env = Environment(
        loader=FileSystemLoader(template_path),
        undefined=StrictUndefined,
        autoescape=False,
    )

    log.debug(template_path)
