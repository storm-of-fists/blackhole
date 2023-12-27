load("//tools/python:defs.bzl", "py_binary")

def _run_notebook_impl(ctx):
    runfiles = ctx.runfiles()
    file_to_run = ctx.attr.py_binary

    runfiles = runfiles.merge(file_to_run.default_runfiles)
    run_file = ctx.actions.declare_file("run.sh")
    run_file_contents = "./" + file_to_run.files_to_run.executable.short_path + " --ip=0.0.0.0 --port=25252 --NotebookApp.token='' --NotebookApp.password='' \n"

    ctx.actions.write(
        content = run_file_contents,
        is_executable = True,
        output = run_file,
    )

    runfiles = runfiles.merge(ctx.runfiles([run_file]))

    return DefaultInfo(
        executable = run_file,
        runfiles = runfiles,
    )

run_notebook = rule(
    attrs = {
        "py_binary": attr.label(
            executable = True,
            cfg = "exec",
        ),
    },
    executable = True,
    implementation = _run_notebook_impl,
)

def py_notebook(name, deps = []):
    deps.append("@external_py//jupyterlab")
    py_binary(
        name = "{}.py_binary".format(name),
        srcs = ["jupyter.py"],
        main = "jupyter.py",
        deps = deps,
    )

    run_notebook(
        name = name,
        py_binary = ":{}.py_binary".format(name),
        tags = [
            "manual",
            "no-sandbox",
            "no-cache",
            "no-remote",
            "local",
            "requires-network",
        ],
    )
