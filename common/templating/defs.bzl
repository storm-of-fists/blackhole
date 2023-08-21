def _render_jinja_templates_impl(ctx):
    ### GET ALL INPUT VARIABLES ###
    name = ctx.label.name

    # Get template types
    string_templates = ctx.attr.string_templates
    srcs = ctx.attr.srcs

    # Get configs
    yaml_config = ctx.attr.yaml_config
    json_config = ctx.attr.json_config
    dict_string_config = ctx.attr.dict_string_config

    # Get renderer settings
    combined_file_type = ctx.attr.combined_file_type
    only_combined_file = ctx.attr.only_combined_file
    combine_mixed_srcs = ctx.attr.combine_mixed_srcs

    # Check that all input files are the same type if want a combined file. Warn if not.
    all_files_same = True
    for i in range(0, len(srcs) - 1):
        all_files_same = srcs[i].extension == srcs[i + 1].extension

    if not all_files_same and combined_file_type and not combine_mixed_srcs:
        fail(
            """Not all input files are of the same type, but a combined_file was requested without an override!"
            Please specify combine_mixed_srcs=True if you want to do this""",
        )

    if not combined_file_type:
        combined_file_type = srcs[0].files.to_list()[0].extension

    # We cant have a combined file without a type.
    if only_combined_file and not combined_file_type:
        fail("Cant ask for only the combined file but not specify a type!")

    ### DECLARE INITIAL VALUES ###
    template_list = []
    rendered_list = []
    config_list = []

    ### CREATE THE CONFIG FILES ###
    # Create a json file for any raw dict config.
    if dict_string_config:
        config_file = ctx.actions.declare_file("{}/config_files/_dict_string_config.json".format(name))
        ctx.actions.write(
            output = config_file,
            content = json.encode(dict_string_config),
        )
        config_list.append(config_file)

    # Create a dir to collect all yaml config files.
    for file_group in yaml_config:
        for yaml_file in file_group.files.to_list():
            config_symlink = ctx.actions.declare_file("{}/config_files/{}".format(name, yaml_file.basename))
            ctx.actions.symlink(output = config_symlink, target_file = yaml_file)
            config_list.append(config_symlink)

    # Create an empty config file just in case.
    empty_config = ctx.actions.declare_file("{}/config/_empty.yaml".format(name))
    ctx.actions.write(output = empty_config, content = "")
    config_list.append(empty_config)

    ### COLLECT TEMPLATES ###
    for template_name, template_str in string_templates.items():
        template_file = ctx.actions.declare_file("{}/templates/{}".format(name, template_name))
        ctx.actions.write(
            output = template_file,
            content = template_str,
        )
        template_list.append(template_file)

    for file_group in srcs:
        for template_file in file_group.files.to_list():
            template_symlink = ctx.actions.declare_file("{}/templates/{}".format(name, template_file.basename))
            ctx.actions.symlink(output = template_symlink, target_file = template_file)
            template_list.append(template_symlink)

    # Now that we have collected all templates, check that we actually have some.
    if not template_list:
        fail("No templates found!")

    ### CREATE RENDERED FILES ###
    if not only_combined_file:
        for template in [template.basename for template in template_list]:
            rendered_file = ctx.actions.declare_file("{}/rendered/{}".format(name, template))
            rendered_list.append(rendered_file)

    if combined_file_type:
        combined_file = ctx.actions.declare_file("{}/rendered/combined.{}".format(name, combined_file_type))
        rendered_list.append(combined_file)

    ### CREATE PY BINARY RENDERING ARGS ###
    exec_args = ctx.actions.args()
    exec_args.add(template_list[0].dirname)
    exec_args.add(rendered_list[0].dirname)
    exec_args.add(config_list[0].dirname)

    if combined_file_type:
        exec_args.add("--combined_file_type={}".format(combined_file_type))
    if only_combined_file:
        exec_args.add("--only_combined_file={}".format(only_combined_file))

    exec_args.add("--block_start_string={}".format(ctx.attr.block_start_string))
    exec_args.add("--block_end_string={}".format(ctx.attr.block_end_string))
    exec_args.add("--variable_start_string={}".format(ctx.attr.variable_start_string))
    exec_args.add("--variable_end_string={}".format(ctx.attr.variable_end_string))
    exec_args.add("--comment_start_string={}".format(ctx.attr.comment_start_string))
    exec_args.add("--comment_end_string={}".format(ctx.attr.comment_end_string))
    exec_args.add("--line_statement_prefix={}".format(ctx.attr.line_statement_prefix))
    exec_args.add("--line_comment_prefix={}".format(ctx.attr.line_comment_prefix))

    ### RUN PY BINARY TO RENDER THINGS ###
    ctx.actions.run(
        executable = ctx.executable._template_renderer_binary,
        arguments = [exec_args],
        inputs = template_list + config_list,
        outputs = rendered_list,
    )

    return [DefaultInfo(files = depset(rendered_list))]

render_jinja_templates = rule(
    implementation = _render_jinja_templates_impl,
    attrs = {
        # Template types
        "string_templates": attr.string_dict(default = {}),
        "srcs": attr.label_list(allow_files = True),

        ### Config types ###
        "yaml_config": attr.label_list(default = [], allow_files = True),
        "json_config": attr.label_list(default = [], allow_files = True),
        "dict_string_config": attr.string(default = ""),

        ### Jinja settings ###
        # Blocks can be instantiated like {% for var in vars %}
        "block_start_string": attr.string(default = "{%"),
        "block_end_string": attr.string(default = "%}"),
        # Variables can be captured like {{ var }}
        "variable_start_string": attr.string(default = "{{"),
        "variable_end_string": attr.string(default = "}}"),
        # Can create comments like {# my comment in the template #}
        "comment_start_string": attr.string(default = "{#"),
        "comment_end_string": attr.string(default = "#}"),
        # Can write blocks like ##/% for var in vars
        "line_statement_prefix": attr.string(default = "##/%"),
        "line_comment_prefix": attr.string(default = "##/#"),
        # If the rendering will fail if the expected variables are not present
        "strict": attr.bool(default = True),

        ### Renderer Settings ###
        # Other settings for the renderer
        "default_config_name": attr.string(default = "config"),
        # Type of the combined file.
        "combined_file_type": attr.string(),
        # If we should only output one combined file.
        "only_combined_file": attr.bool(default = False),
        "combine_mixed_srcs": attr.bool(default = False),

        ### Binary Path ###
        # Rendering binary, probably dont touch this.
        "_template_renderer_binary": attr.label(
            default = Label("//common/templating:jinja_template_renderer"),
            executable = True,
            cfg = "exec",
        ),
    },
)

def config(
        srcs,
        yaml_config = [],
        block_start_string = "{%",
        block_end_string = "%}",
        variable_start_string = "{{",
        variable_end_string = "}}",
        **kwargs):
    render_jinja_templates(
        srcs = srcs,
        yaml_config = yaml_config,
        only_combined_file = True,
        block_start_string = block_start_string,
        block_end_string = block_end_string,
        variable_start_string = variable_start_string,
        variable_end_string = variable_end_string,
        **kwargs
    )
