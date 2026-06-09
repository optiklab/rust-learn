load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

# This BUILD file is meant to be run from the Crubit Bazel workspace.
# See README.md for the copy/sync command from this learning repo into:
#   D:/my-projects/Rust/crubit/examples/56_crubit

licenses(["notice"])

# Direction 1: Rust calls C++.
#
# Crubit sees this C++ target because aspect_hints opts it into the supported
# C++-to-Rust binding generator. The generated Rust crate will be named after
# this target: cpp_temperature.
cc_library(
    name = "cpp_temperature",
    hdrs = ["cpp_temperature.h"],
    aspect_hints = ["//features:supported"],
)

rust_binary(
    name = "rust_calls_cpp",
    srcs = ["rust_calls_cpp.rs"],

    # cc_deps is Crubit's main difference from ordinary Rust build rules:
    # this Rust binary can import generated Rust bindings for cpp_temperature.
    cc_deps = [":cpp_temperature"],
)

# Direction 2: C++ calls Rust.
#
# A normal Rust library can receive generated C++ bindings through the
# cc_bindings_from_rust rule below.
rust_library(
    name = "rust_rules",
    srcs = ["rust_rules.rs"],
)

cc_bindings_from_rust(
    name = "rust_rules_cc_api",
    crate = ":rust_rules",
)

cc_binary(
    name = "cpp_calls_rust",
    srcs = ["cpp_calls_rust.cc"],
    deps = [":rust_rules_cc_api"],
)