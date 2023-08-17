load("//tools/jinja:defs.bzl", _render_jinja_templates = "render_jinja_templates")

render_jinja_templates = _render_jinja_templates

def config(srcs, config, **kwargs):
    render_jinja_templates(yaml_templates = srcs, yaml_config = config, combined_file_type = "yaml", only_combined_file = True, **kwargs)
