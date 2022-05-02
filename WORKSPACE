# Copyright 2022 The Firefly Authors.
#
# Use of this source code is governed by a BSD 3-clause
# license that can be found in the LICENSE file.

workspace(name = "sample")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "1c531376ac7e5a180e0237938a2536de0c54d93f5c278634818e0efc952dd56c",
    urls = [
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.3/bazel-skylib-1.0.3.tar.gz",
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.3/bazel-skylib-1.0.3.tar.gz",
    ],
)

http_archive(
    name = "rules_rust",
    sha256 = "ab3de8c51a210bae2987bd0d3379b84d50c0e053219fad14969a93a842f1eabc",
    strip_prefix = "rules_rust-c3f56c2d50c29c97c513f158bcb1dfef1bd52f1e",
    urls = ["https://github.com/bazelbuild/rules_rust/archive/c3f56c2d50c29c97c513f158bcb1dfef1bd52f1e.tar.gz"],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    include_rustc_srcs = True,
    iso_date = "2022-04-10",
    rustfmt_version = "nightly",
    sha256s = {
        "2022-02-01/llvm-tools-nightly-x86_64-unknown-linux-gnu": "3ccc356d9d7a4415790db539aa1c449d77b75d249732bbe0cb3248a5f39e428d",
        "2022-02-01/rust-nightly-x86_64-unknown-linux-gnu": "05af4d844b308bfee0baa0f61a977a928b6b7eb27d4c859ececed5cab83a055d",
        "2022-02-01/rust-src-nightly": "19bd1a6030c98643ed270682b031997fb323fc90fefc72fe2cb313e256ab0016",
        "2022-02-01/rust-std-nightly-x86_64-unknown-linux-gnu": "4166a60222de2c491847c3c925bcaf341afb19cad512f1c702b3b48e90867c90",
        "2022-02-01/rustfmt-nightly-x86_64-unknown-linux-gnu": "7fe3049fb4003f35539e622801cd62e1d20481915e4879aeb47965dafeb859bf",
    },
    version = "nightly",
)
