load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")
load("//third_party/rust:crates.bzl", "third_party_crates")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("@third_party_rust//:defs.bzl", "crate_repositories")
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

def _rust_deps(_ctx):
    http_archive(
        name = "rules_rust",
        sha256 = "9d04e658878d23f4b00163a72da3db03ddb451273eb347df7d7c50838d698f49",
        urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.26.0/rules_rust-v0.26.0.tar.gz"],
    )



    rules_rust_dependencies()

    RUST_STABLE_VERSION = "1.72.0"

    RUST_NIGHTLY_VERSION = "nightly/2023-08-26"

    rust_register_toolchains(
        edition = "2021",
        rust_analyzer_version = RUST_STABLE_VERSION,
        rustfmt_version = RUST_STABLE_VERSION,
        versions = [
            RUST_STABLE_VERSION,
            RUST_NIGHTLY_VERSION,
        ],
    )

    crate_universe_dependencies()

    crates_repository(
        name = "third_party_rust",
        cargo_lockfile = "//third_party/rust:Cargo.Bazel.lock",
        lockfile = "//third_party/rust:cargo-bazel-lock.json",
        packages = {
            name: crate.spec(
                default_features = info.get("default_features", True),
                features = info.get("features", []),
                version = info["version"],
            )
            for name, info in third_party_crates.items()
        },
        # Setting the default package name to `""` forces the use of the macros defined in this repository
        # to always use the root package when looking for dependencies or aliases. This should be considered
        # optional as the repository also exposes alises for easy access to all dependencies.
        render_config = render_config(
            default_package_name = "",
        ),
        rust_version = RUST_STABLE_VERSION,
    )

    crate_repositories()

    rust_analyzer_dependencies()

rust_deps = module_extension(
    implementation = _rust_deps,
)
