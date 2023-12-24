"""Get the ephemerides for the solar system."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

def _download_de440(_ctx):
    http_file(
        name = "de440",
        url = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440.bsp",
        integrity = "sha256-pM6b+bMoK+zJ9LKsPOvgOirnWZmBqr1yZf2Egv/3xLU=",
    )

download_de440 = module_extension(
    implementation = _download_de440,
)
