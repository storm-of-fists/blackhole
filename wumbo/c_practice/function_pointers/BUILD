cc_library(
    name = "function_pointers",
    srcs = [
        "function_pointers.c",
    ],
    hdrs = ["function_pointers.c",],
)

cc_binary(
    name = "run",
    deps = [":function_pointers"]
)

# did some manual cross compilation https://opensource.com/article/19/7/cross-compiling-gcc
