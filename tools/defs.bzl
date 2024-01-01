load("//tools/python:defs.bzl", _py_binary = "py_binary")
load("@rules_proto//proto:defs.bzl", _proto_library = "proto_library")
load("@grpc//bazel:python_rules.bzl", _py_proto_library = "py_proto_library")
load("@rules_oci//oci:defs.bzl", _oci_image = "oci_image", _oci_push = "oci_push")
load("@rules_pkg//pkg:tar.bzl", _pkg_tar = "pkg_tar")
load("@rules_pkg//pkg:mappings.bzl", _pkg_files = "pkg_files")

proto_library = _proto_library
py_proto_library = _py_proto_library

py_binary = _py_binary

oci_image = _oci_image
oci_push = _oci_push
pkg_tar = _pkg_tar
pkg_files = _pkg_files
