def _run_binary_impl(ctx):
    runfiles = ctx.runfiles()
    file_to_run = ctx.attr.run

    # collect the runfiles of the other executables so their own runfiles
    # will be available when the top-level executable runs
    runfiles = runfiles.merge(file_to_run.default_runfiles)

    run_file = ctx.actions.declare_file("run.sh")

    ctx.actions.write(
        # the "./" is needed if the executable is in the current directory
        # (i.e. in the workspace root)
        content = "./" + file_to_run.files_to_run.executable.short_path + "\n",
        is_executable = True,
        output = run_file,
    )

    runfiles = runfiles.merge(ctx.runfiles([run_file]))

    return DefaultInfo(
        executable = run_file,
        runfiles = runfiles,
    )

run_binary = rule(
    attrs = {
        "run": attr.label(),
    },
    executable = True,
    implementation = _run_binary_impl,
)
