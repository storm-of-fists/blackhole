def _render_jinja_template_impl(ctx):
    name = ctx.label.name
    string_template = ctx.attr.string_template
    dict_config = ctx.attr.dict_config

    if string_template:
        template_file = ctx.actions.declare_file("generated_template_file_{}".format(name))
        ctx.actions.write(
            output = template_file,
            content = string_template,
        )

    rendered_template = ctx.actions.declare_file("{}".format(name))
    outputs = [rendered_template]

    args = ctx.actions.args()
    args.add("{}".format(template_file.path))
    args.add(dict_config)

    ctx.actions.run(
        executable = ctx.executable._template_renderer_binary,
        arguments = [args],
        inputs = depset([template_file]),
        outputs = outputs
    )

    return [DefaultInfo(files = depset(outputs))]

render_jinja_template = rule(
    implementation = _render_jinja_template_impl,
    attrs = {
        "string_template": attr.string(),
        "yaml_template": attr.label(),
        "dict_config": attr.string_dict(),
        "yaml_config": attr.label(),
        "_template_renderer_binary": attr.label(
            default = Label("//tools/jinja:jinja_template_renderer"),
            executable = True,
            cfg = "exec",
        )
    }
)
