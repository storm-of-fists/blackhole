import argparse
import jinja2
from jinja2 import Environment, StrictUndefined, FileSystemLoader
from pathlib import Path

# import ast
import yaml
import json

if __name__ == "__main__":
    # Parse args
    parser = argparse.ArgumentParser()
    parser.add_argument("templates", type=Path)
    parser.add_argument("rendered", type=Path)
    parser.add_argument("config", type=Path)
    parser.add_argument("--combined_file_type", type=str, default=None)
    parser.add_argument("--only_combined_file", type=bool, default=False)
    parser.add_argument("--config_name", type=str, default="")
    args = parser.parse_args()

    # Set up directories for the outputs
    template_dir = args.templates
    rendered_dir = args.rendered
    config_dir = args.config

    combined_file_type = args.combined_file_type
    only_combined_file = args.only_combined_file
    config_name = args.config_name

    # Configure jinja env
    jinja_env = Environment(
        loader=FileSystemLoader(template_dir),
        undefined=StrictUndefined,
        autoescape=False,
        trim_blocks=True,
        lstrip_blocks=True,
    )

    combined_config = {}

    for config_file in config_dir.iterdir():
        config_dict = {}

        if config_file.suffix == ".yaml":
            with config_file.open() as yaml_file:
                # TODO check for duplicate keys here
                config_dict = yaml.load(yaml_file, Loader=yaml.FullLoader)
        elif config_file.suffix == ".json":
            with config_file.open() as json_file:
                # TODO check for duplicate keys here
                config_dict = json.loads(json.load(json_file))

        if config_dict:
            combined_config.update(config_dict)

    if combined_file_type:
        combined_rendered = rendered_dir / f"combined.{combined_file_type}"
        combined_text = ""

    # Render each template, write to rendered dir
    for template_name in jinja_env.list_templates():
        template = jinja_env.get_template(template_name)

        rendered_template = rendered_dir / template_name

        try:
            if config_name:
                rendered_text = template.render(**{f"{config_name}": combined_config})
            else:
                rendered_text = template.render(**combined_config)
        except jinja2.exceptions.UndefinedError as undefined_err:
            raise Exception(
                "You seemed to have used an incorrect string when referencing config "
                "in one of your templates! Please ensure you reference other config "
                f"with the correct keyword: {undefined_err}"
            )
        except Exception as err:
            raise Exception(f"Seems like we dont capture this error: {err}")

        if not only_combined_file:
            rendered_template.write_text(rendered_text)

        if combined_file_type:
            combined_text += rendered_text + "\n"

    if combined_file_type:
        combined_rendered.write_text(combined_text)
