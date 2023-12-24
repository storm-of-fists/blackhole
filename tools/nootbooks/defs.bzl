load("//tools/python:defs.bzl", "py_binary")

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


def _py_notebook_runner_impl(ctx):
    bin = ctx.attr.py_bin

    ctx.actions.run(
        inputs = bin.files,
        outputs = [ctx.outputs.executable],
        # command = "{} --ip=0.0.0.0 --port=25252 --NotebookApp.token='' --NotebookApp.password=''".format(bin_file.path),
        executable = ctx.executable.py_bin,
    )

py_notebook_runner = rule(
    implementation = _py_notebook_runner_impl,
    executable = True,
    attrs = {
        "py_bin": attr.label(
            executable = True,
            cfg = "exec",
        ),
    },
)

def py_notebook(name, deps):
    py_binary(
        name = "{}.py_binary".format(name),
        srcs = ["//tools/python:jupyter.py"],
        main = "//tools/python:jupyter.py",
        deps = deps,
        tags = ["manual"],
    )

    py_notebook_runner(
        name = name,
        py_bin = ":{}.py_binary".format(name),
        tags = [
            "manual",
            "no-sandbox",
            "no-cache",
            "no-remote",
            "local",
            "requires-network",
        ],
    )