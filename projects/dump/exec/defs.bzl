def _impl1(ctx):
    ctx.actions.write(
        content = "echo %s 123" % ctx.label.name,
        is_executable = True,
        output = ctx.outputs.executable,
    )
    return DefaultInfo(executable = ctx.outputs.executable)

exec_rule1 = rule(
    executable = True,
    implementation = _impl1,
)

def _impl2(ctx):
    executable_paths = []
    runfiles = ctx.runfiles()
    for dep in ctx.attr.deps:
        # the "./" is needed if the executable is in the current directory
        # (i.e. in the workspace root)
        executable_paths.append("./" + dep.files_to_run.executable.short_path)

        # collect the runfiles of the other executables so their own runfiles
        # will be available when the top-level executable runs
        runfiles = runfiles.merge(dep.default_runfiles)

    ctx.actions.write(
        content = "\n".join(executable_paths),
        is_executable = True,
        output = ctx.outputs.executable,
    )

    return DefaultInfo(
        executable = ctx.outputs.executable,
        runfiles = runfiles,
    )

exec_rule2 = rule(
    attrs = {
        "deps": attr.label_list(),
    },
    executable = True,
    implementation = _impl2,
)
